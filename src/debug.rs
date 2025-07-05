use std::{
    cell::RefCell,
    ops::Range,
    sync::{
        atomic::{AtomicBool, AtomicPtr},
        Arc, LazyLock, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

use circular_buffer::CircularBuffer;

use crate::{
    config::config, linker::FileData, prelude::Span, pretty_print_span,
    pretty_print_spans_in_reverse_order,
};

/// Many duplicates will be produced, and filtering them out in the code itself is inefficient. Therefore just keep a big buffer and deduplicate as needed
const SPAN_TOUCH_HISTORY_SIZE: usize = 256;
const RECENT_DEBUG_FLAG_SIZE: usize = 10;
const NUM_SPANS_TO_PRINT: usize = 10;

struct PerThreadDebugInfo {
    debug_stack: Vec<SpanDebuggerStackElement>,
    recent_debug_options: CircularBuffer<RECENT_DEBUG_FLAG_SIZE, &'static str>,
}

struct SpanDebuggerStackElement {
    context: String,
    debugging_enabled: bool,
    span_history: CircularBuffer<SPAN_TOUCH_HISTORY_SIZE, Span>,
}

thread_local! {
    static DEBUG_STACK : RefCell<PerThreadDebugInfo> = const { RefCell::new(PerThreadDebugInfo{debug_stack: Vec::new(), recent_debug_options: CircularBuffer::new()}) };
    static MOST_RECENT_FILE_DATA: std::sync::atomic::AtomicPtr<FileData> = const {AtomicPtr::new(std::ptr::null_mut())}
}

/// Register a [crate::file_position::Span] for potential printing by [PanicGuardSpanPrinter] on panic.
///
/// Would like to use [crate::file_position::Span], but cannot copy the span because that would create infinite loop.
///
/// So use [Range] instead.
pub fn add_debug_span(sp: Span) {
    // Convert to range so we don't invoke any of Span's triggers
    DEBUG_STACK.with_borrow_mut(|history| {
        let Some(last) = history.debug_stack.last_mut() else {
            return; // Can't track Spans not in a SpanDebugger region
        };

        last.span_history.push_back(sp);
    });
}

fn print_most_recent_spans(file_data: &FileData, history: SpanDebuggerStackElement) {
    let mut spans_to_print: Vec<Range<usize>> = Vec::with_capacity(NUM_SPANS_TO_PRINT);

    for sp in history.span_history.iter().rev() {
        let as_range = sp.as_range();
        if !spans_to_print.contains(&as_range) {
            spans_to_print.push(as_range);
        }
        if spans_to_print.len() >= NUM_SPANS_TO_PRINT {
            break;
        }
    }

    println!("Panic unwinding. Printing the last {} spans. BEWARE: These spans may not correspond to this file, thus incorrect spans are possible!", spans_to_print.len());
    pretty_print_spans_in_reverse_order(file_data, spans_to_print);
}

#[allow(unused)]
pub fn debug_print_span(span: Span, label: String) {
    MOST_RECENT_FILE_DATA.with(|ptr| {
        let ptr = ptr.load(std::sync::atomic::Ordering::SeqCst);
        if ptr.is_null() {
            eprintln!("No FileData registered!");
        } else {
            let fd: &FileData = unsafe { &*ptr };
            pretty_print_span(fd, span.as_range(), label);
        }
    })
}

/// Print the last [NUM_SPANS_TO_PRINT] touched spans on panic to aid in debugging
///
/// If not defused, it will print when dropped, ostensibly when being unwound from a panic
///
/// Must call [Self::defuse] when no panic occurred
///
/// This struct uses a shared thread_local resource: [SPANS_HISTORY], so no two can exist at the same time (within the same thread).
///
/// Maybe future work can remove dependency on Linker lifetime with some unsafe code.
pub struct SpanDebugger<'text> {
    file_data: &'text FileData,
    stage: &'static str,
    started_at: std::time::Instant,
    /// Kills the process if a SpanDebugger lives longer than config.kill_timeout
    #[allow(unused)]
    out_of_time_killer: OutOfTimeKiller,
}

fn print_stack_top(enter_exit: &str) {
    DEBUG_STACK.with_borrow(|stack| {
        if !config().enabled_debug_paths.contains("spandebugger") {
            return;
        }
        if let Some(top) = stack.debug_stack.last() {
            let debug_enabled = if top.debugging_enabled {
                " DEBUGGING ENABLED"
            } else {
                ""
            };
            println!(
                "{enter_exit}SpanDebugger (x{}) {}{debug_enabled}",
                stack.debug_stack.len(),
                top.context
            );
        } else {
            println!("SpanDebugger (x0)")
        }
    })
}

impl<'text> SpanDebugger<'text> {
    pub fn new(stage: &'static str, global_obj_name: &str, file_data: &'text FileData) -> Self {
        MOST_RECENT_FILE_DATA.with(|ptr| {
            let file_data = file_data as *const FileData;
            ptr.store(
                file_data as *mut FileData,
                std::sync::atomic::Ordering::SeqCst,
            )
        });
        let context = format!("{stage} {global_obj_name}");
        let config = config();

        DEBUG_STACK.with_borrow_mut(|history| {
            let debugging_enabled = config.debug_whitelist.is_empty()
                && !config.enabled_debug_paths.is_empty()
                || config
                    .debug_whitelist
                    .iter()
                    .any(|v| global_obj_name.contains(v));

            history.debug_stack.push(SpanDebuggerStackElement {
                context: context.clone(),
                debugging_enabled,
                span_history: CircularBuffer::new(),
            });
        });
        print_stack_top("Enter ");

        Self {
            file_data,
            stage,
            started_at: std::time::Instant::now(),
            out_of_time_killer: OutOfTimeKiller::new(context),
        }
    }
}

impl Drop for SpanDebugger<'_> {
    fn drop(&mut self) {
        let time_taken = std::time::Instant::now() - self.started_at;
        print_stack_top(&format!(
            "Exit (Took {})",
            humantime::format_duration(time_taken)
        ));

        let last_stack_elem = DEBUG_STACK
            .with_borrow_mut(|stack| stack.debug_stack.pop())
            .unwrap();

        print_stack_top("");
        if std::thread::panicking() {
            eprintln!(
                "Panic happened in Span-guarded context {} in {}",
                self.stage, last_stack_elem.context
            );
            print_most_recent_spans(self.file_data, last_stack_elem)
        }
    }
}

/// Check if the debug path is enabled
pub fn is_enabled(path_id: &'static str) -> bool {
    DEBUG_STACK.with_borrow_mut(|stack| {
        stack.recent_debug_options.push_back(path_id);
        let Some(last) = stack.debug_stack.last() else {
            return false;
        };
        if !last.debugging_enabled {
            return false;
        }

        config().enabled_debug_paths.contains(path_id)
    })
}

#[allow(unused)]
pub fn debugging_enabled() -> bool {
    DEBUG_STACK.with_borrow_mut(|stack| {
        let Some(last) = stack.debug_stack.last() else {
            return false;
        };
        last.debugging_enabled
    })
}

pub fn setup_panic_handler() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        default_hook(info);

        DEBUG_STACK.with_borrow(|history| {
            println!("Most recent available debug paths:");
            for d in &history.recent_debug_options {
                println!("--debug {d}");
            }
        })
    }));
}

struct TimerEntry {
    started_at: Instant,
    info: String,
    alive: Arc<AtomicBool>,
}

static WATCHDOG: LazyLock<Mutex<Vec<TimerEntry>>> = LazyLock::new(|| {
    let list = Mutex::new(Vec::new());
    spawn_watchdog_thread();
    list
});

fn spawn_watchdog_thread() {
    let duration = config().kill_timeout;
    if duration.is_zero() {
        return; // No watchdog
    }
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(50));

        let mut timers = WATCHDOG.lock().unwrap();
        let now = Instant::now();
        timers.retain(|entry| {
            let deadline = entry.started_at + duration;
            if deadline <= now && entry.alive.load(std::sync::atomic::Ordering::SeqCst) {
                println!("⏰⏰⏰⏰⏰⏰⏰⏰⏰"); // To show in stdout when this happens too
                eprintln!(
                    "⏰ OutOfTimeKiller triggered in {} after it took more than {} to execute ⏰",
                    entry.info,
                    humantime::format_duration(now - entry.started_at)
                );
                eprintln!("Process will now be terminated.");
                std::process::exit(1);
            } else {
                deadline > now
            }
        });
    });
}

pub struct OutOfTimeKiller {
    alive: Arc<AtomicBool>,
}

impl OutOfTimeKiller {
    pub fn new(info: String) -> Self {
        let timeout = config().kill_timeout;
        let alive = Arc::new(AtomicBool::new(true));
        if !timeout.is_zero() {
            WATCHDOG.lock().unwrap().push(TimerEntry {
                started_at: Instant::now(),
                alive: alive.clone(),
                info,
            });
        }

        OutOfTimeKiller { alive }
    }
}

impl Drop for OutOfTimeKiller {
    fn drop(&mut self) {
        self.alive.store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

#[macro_export]
macro_rules! __debug_span {
    ($span:expr) => {
        if $crate::debug::debugging_enabled() {
            let tmp = $span;
            std::eprintln!(
                "[{}:{}:{}] {}:",
                std::file!(),
                std::line!(),
                std::column!(),
                std::stringify!($span)
            );

            $crate::debug::debug_print_span(tmp, String::new());
        }
    };
    ($span:expr, $($arg:tt)*) => {
        if $crate::debug::debugging_enabled() {
            let tmp = $span;
            std::eprintln!(
                "[{}:{}:{}] {}:",
                std::file!(),
                std::line!(),
                std::column!(),
                std::stringify!($span)
            );

            $crate::debug::debug_print_span(tmp, format!($($arg)*));
        }
    };
}
