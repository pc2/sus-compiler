use crate::{prelude::*, to_string::join_shorten_filename};

use std::{
    cell::RefCell,
    ops::Range,
    path::PathBuf,
    sync::{
        Arc, LazyLock, Mutex,
        atomic::{AtomicBool, AtomicPtr},
    },
    thread,
    time::{Duration, Instant},
};

use circular_buffer::CircularBuffer;
use colored::Colorize;

use crate::{
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

#[derive(Debug)]
struct SpanDebuggerStackElement {
    stage: &'static str,
    global_obj_name: String,
    debugging_enabled: bool,
    span_history: CircularBuffer<SPAN_TOUCH_HISTORY_SIZE, Span>,
    file_data: *const FileData,
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

    info!(
        "Panic unwinding. Printing the last {} spans. BEWARE: These spans may not correspond to this file, thus incorrect spans are possible!",
        spans_to_print.len()
    );
    pretty_print_spans_in_reverse_order(file_data, spans_to_print);
}

#[allow(unused)]
pub fn debug_print_span(span: Span, label: String) {
    MOST_RECENT_FILE_DATA.with(|ptr| {
        let ptr = ptr.load(std::sync::atomic::Ordering::SeqCst);
        if ptr.is_null() {
            error!("No FileData registered for Span Debugging!");
        } else {
            let fd: &FileData = unsafe { &*ptr };
            pretty_print_span(fd, span, label);
        }
    })
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
            trace!(
                "{enter_exit}SpanDebugger (x{}) {} for {}{debug_enabled}",
                stack.debug_stack.len(),
                top.stage,
                top.global_obj_name
            );
        } else {
            trace!("SpanDebugger (x0)")
        }
    })
}

pub fn debug_context<R>(
    stage: &'static str,
    global_obj_name: String,
    file_data: &FileData,
    f: impl FnOnce() -> R,
) -> R {
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
            file_data,
        });
    });
    print_stack_top("Enter ");

    let started_at = std::time::Instant::now();
    let _out_of_time_killer = OutOfTimeKiller::new(oot_killer_str);

    let result = f(); // On panic, skips the following code (of course _out_of_time_killer is still Dropped)

    let time_taken = std::time::Instant::now() - started_at;
    print_stack_top(&format!("Exit (Took {:.2}s)", time_taken.as_secs_f64()));

    // Clean up most recent debug stack frame. DOES NOT TRIGGER ON PANIC, such that it doesn't destroy the state for [setup_panic_handler]
    DEBUG_STACK.with_borrow_mut(|stack| {
        let _ = stack.debug_stack.pop().unwrap();
    });

    result
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
            if let Some(last_stack_elem) = history.debug_stack.last() {
                info!(
                    "Panic happened in Span-guarded context {} in {}",
                    last_stack_elem.stage.red(),
                    last_stack_elem.global_obj_name.red()
                );
                let file_data = unsafe { &*last_stack_elem.file_data };
                //pretty_print_span(file_data, span, label);
                print_most_recent_spans(file_data, last_stack_elem);
            }
            info!("Most recent available debug paths:");
            for (ctx, d) in &history.recent_debug_options {
                if let Some(ctx) = ctx {
                    info!("--debug-whitelist {ctx} --debug {d}");
                } else {
                    info!("(no SpanDebugger Context) --debug {d}");
                }
            }
        })
    }));
}

pub fn create_dump_on_panic<R>(linker: &mut Linker, f: impl FnOnce(&mut Linker) -> R) -> R {
    if crate::config::config().no_redump {
        // Run without protection, don't create a dump on panic
        return f(linker);
    }

    use std::fs;
    use std::io::Write;

    let panic_info = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(linker))) {
        Ok(result) => return result,
        Err(panic_info) => panic_info,
    };
    // Get ~/.sus/crash_dumps/{timestamp}
    let cur_time = chrono::Local::now()
        .format("_%Y-%m-%d_%H:%M:%S")
        .to_string();

    let failure_name = DEBUG_STACK.with_borrow(|history| {
        if let Some(SpanDebuggerStackElement {
            stage,
            global_obj_name,
            ..
        }) = history.debug_stack.last()
        {
            let global_obj_name = global_obj_name.replace(char::is_whitespace, "");
            format!("{stage}_{global_obj_name}",)
        } else {
            "unknown".to_string()
        }
    });

    let dump_name = join_shorten_filename(&failure_name, &cur_time);
    let mut dump_dir = config().sus_home.join("crash_dumps").join(&dump_name);

    if let Err(err) = fs::create_dir_all(&dump_dir) {
        let new_dump_dir = PathBuf::from("sus_crash_dumps").join(&dump_name);
        error!(
            "Could not create {} in the SUS install directory: {err} Trying to save it locally to {}",
            dump_dir.to_string_lossy(),
            new_dump_dir.to_string_lossy()
        );

        if let Err(err) = fs::create_dir_all(&new_dump_dir) {
            error!(
                "Could not create {} locally either: {err} Giving up on dumping the error",
                new_dump_dir.to_string_lossy()
            );

            std::panic::resume_unwind(panic_info);
        }

        dump_dir = new_dump_dir;
    }

    // Write reproduce.sh with compiler args
    let args: Vec<String> = std::env::args().collect();
    let reproduce_path = dump_dir.join("reproduce.sh");
    if let Ok(mut f) = fs::File::create(&reproduce_path) {
        use crate::config::VERSION_INFO;
        let cmd = format!(
            "#!/bin/sh\n#SUS Compiler Version: {VERSION_INFO}\n{}\n",
            args.join(" ")
        );
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
    error!("Internal compiler error! All files dumped to {dump_dir:?}");
    std::panic::resume_unwind(panic_info);
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
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(50));

            let mut timers = WATCHDOG.lock().unwrap();
            let now = Instant::now();
            timers.retain(|entry| {
            let deadline = entry.started_at + duration;
            if deadline <= now && entry.alive.load(std::sync::atomic::Ordering::SeqCst) {
                fatal_exit!("⏰⏰⏰⏰⏰⏰⏰⏰⏰\n⏰ OutOfTimeKiller triggered in {} after it took more than {:.2} seconds to execute ⏰\nProcess will now be terminated.", 
                    entry.info,
                    (now - entry.started_at).as_secs_f64()); // To show in stdout when this happens too
            } else {
                deadline > now
            }
        });
        }
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
            eprintln!(
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
            eprintln!(
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

#[macro_export]
macro_rules! __debug_dbg {
    ($($arg:tt)*) => {
        if $crate::debug::debugging_enabled() {
            dbg!($($arg)*);
        }
    };
}
