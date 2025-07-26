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
use colored::Colorize;

use crate::{
    compiler_top::get_core_dumps_dir,
    config::config,
    linker::{FileData, Linker},
    prelude::Span,
    pretty_print_span, pretty_print_spans_in_reverse_order,
};

/// Many duplicates will be produced, and filtering them out in the code itself is inefficient. Therefore just keep a big buffer and deduplicate as needed
const SPAN_TOUCH_HISTORY_SIZE: usize = 256;
const RECENT_DEBUG_FLAG_SIZE: usize = 10;
const NUM_SPANS_TO_PRINT: usize = 10;

struct PerThreadDebugInfo {
    debug_stack: Vec<SpanDebuggerStackElement>,
    recent_debug_options: CircularBuffer<RECENT_DEBUG_FLAG_SIZE, (Option<String>, &'static str)>,
}

struct SpanDebuggerStackElement {
    stage: &'static str,
    global_obj_name: String,
    debugging_enabled: bool,
    span_history: CircularBuffer<SPAN_TOUCH_HISTORY_SIZE, Span>,
}

thread_local! {
    static DEBUG_STACK : RefCell<PerThreadDebugInfo> = const { RefCell::new(PerThreadDebugInfo{debug_stack: Vec::new(), recent_debug_options: CircularBuffer::new()}) };
    static MOST_RECENT_FILE_DATA: std::sync::atomic::AtomicPtr<FileData> = const {AtomicPtr::new(std::ptr::null_mut())}
}

/// Register a [crate::file_position::Span] for printing by [SpanDebugger] on panic.
pub fn add_debug_span(sp: Span) {
    // Convert to range so we don't invoke any of Span's triggers
    DEBUG_STACK.with_borrow_mut(|history| {
        let Some(last) = history.debug_stack.last_mut() else {
            return; // Can't track Spans not in a SpanDebugger region
        };

        last.span_history.push_back(sp);
    });
}

fn print_most_recent_spans(file_data: &FileData, history: &SpanDebuggerStackElement) {
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
                "{enter_exit}SpanDebugger (x{}) {} for {}{debug_enabled}",
                stack.debug_stack.len(),
                top.stage,
                top.global_obj_name
            );
        } else {
            println!("SpanDebugger (x0)")
        }
    })
}

impl<'text> SpanDebugger<'text> {
    pub fn new(stage: &'static str, global_obj_name: String, file_data: &'text FileData) -> Self {
        MOST_RECENT_FILE_DATA.with(|ptr| {
            let file_data = file_data as *const FileData;
            ptr.store(
                file_data as *mut FileData,
                std::sync::atomic::Ordering::SeqCst,
            )
        });
        let config = config();

        let oot_killer_str = format!("{stage} {global_obj_name}");

        DEBUG_STACK.with_borrow_mut(|history| {
            let debugging_enabled = config.debug_whitelist.is_empty()
                && !config.enabled_debug_paths.is_empty()
                || config
                    .debug_whitelist
                    .iter()
                    .any(|v| global_obj_name.contains(v));

            history.debug_stack.push(SpanDebuggerStackElement {
                stage,
                global_obj_name,
                debugging_enabled,
                span_history: CircularBuffer::new(),
            });
        });
        print_stack_top("Enter ");

        Self {
            file_data,
            started_at: std::time::Instant::now(),
            out_of_time_killer: OutOfTimeKiller::new(oot_killer_str),
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

        DEBUG_STACK.with_borrow_mut(|stack| {
            if std::thread::panicking() {
                let last_stack_elem = stack.debug_stack.last().unwrap();
                eprintln!(
                    "Panic happened in Span-guarded context {} in {}",
                    last_stack_elem.stage.red(),
                    last_stack_elem.global_obj_name.red()
                );
                print_most_recent_spans(self.file_data, last_stack_elem)
            } else {
                let _ = stack.debug_stack.pop().unwrap();
            }
        });
        print_stack_top("");
    }
}

/// Check if the debug path is enabled
pub fn is_enabled(path_id: &'static str) -> bool {
    DEBUG_STACK.with_borrow_mut(|stack| {
        let last = stack.debug_stack.last();
        stack
            .recent_debug_options
            .push_back((last.map(|last| last.global_obj_name.clone()), path_id));

        if !matches!(
            last,
            Some(SpanDebuggerStackElement {
                debugging_enabled: true,
                ..
            })
        ) {
            return false;
        };

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
            for (ctx, d) in &history.recent_debug_options {
                if let Some(ctx) = ctx {
                    println!("--debug-whitelist {ctx} --debug {d}");
                } else {
                    println!("(no SpanDebugger Context) --debug {d}");
                }
            }
        })
    }));
}

pub fn create_dump_on_panic(linker: &mut Linker, f: impl FnOnce(&mut Linker)) {
    use std::fs;
    use std::io::Write;
    use std::time::SystemTime;

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(linker)));

    if let Err(panic_info) = result {
        // Get ~/.sus/core_dumps/{timestamp}
        let cur_time = humantime::format_rfc3339(SystemTime::now());

        let failure_name = DEBUG_STACK.with_borrow(|history| {
            if let Some(SpanDebuggerStackElement {
                stage,
                global_obj_name,
                ..
            }) = history.debug_stack.last()
            {
                format!("{stage} {global_obj_name} {cur_time}")
            } else {
                format!("no module {cur_time}")
            }
        });

        let dump_dir = get_core_dumps_dir().join(failure_name);
        if let Err(err) = fs::create_dir_all(&dump_dir) {
            eprintln!("Could not create {}: {err}", dump_dir.to_string_lossy());
            std::panic::resume_unwind(panic_info);
        }

        // Write reproduce.sh with compiler args
        let args: Vec<String> = std::env::args().collect();
        let reproduce_path = dump_dir.join("reproduce.sh");
        if let Ok(mut f) = fs::File::create(&reproduce_path) {
            let cmd = format!("#!/bin/sh\n{}\n", args.join(" "));
            let _ = f.write_all(cmd.as_bytes());
        }

        for (_id, file_data) in &linker.files {
            // Exclude files from the standard library directory
            if file_data.is_std {
                continue;
            }
            let filename = file_data.file_identifier.replace("/", "_");
            let path = dump_dir.join(&filename);
            if let Ok(mut f) = fs::File::create(&path) {
                let _ = f.write_all(file_data.file_text.file_text.as_bytes());
            }
        }
        eprintln!("Internal compiler error! All files dumped to {dump_dir:?}");
        std::panic::resume_unwind(panic_info);
    }
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
