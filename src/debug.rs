use crate::{dev_aid::ariadne_interface::pretty_print_many_spans, linker::LinkerFiles, prelude::*};

use std::{
    cell::{Cell, RefCell},
    path::PathBuf,
    sync::{Arc, LazyLock, Mutex, atomic::AtomicBool},
    thread,
    time::{Duration, Instant},
};

use circular_buffer::CircularBuffer;
use colored::Colorize;

use crate::{
    codegen::sanitize_filename, config::config, dev_aid::ariadne_interface::pretty_print_span,
    linker::Linker,
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
}

thread_local! {
    static DEBUG_STACK : RefCell<PerThreadDebugInfo> = const { RefCell::new(PerThreadDebugInfo{debug_stack: Vec::new(), recent_debug_options: CircularBuffer::new()}) };
    static STORED_LINKER: Cell<*mut Linker> = const { Cell::new(std::ptr::null_mut()) };
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

fn print_most_recent_spans(linker_files: &LinkerFiles, history: &SpanDebuggerStackElement) {
    let mut spans_to_print: Vec<Span> = Vec::with_capacity(NUM_SPANS_TO_PRINT);

    for sp in history.span_history.iter().rev() {
        if !spans_to_print.contains(sp) {
            spans_to_print.push(*sp);
        }
        if spans_to_print.len() >= NUM_SPANS_TO_PRINT {
            break;
        }
    }

    info!(
        "Panic unwinding. Printing the last {} spans.",
        spans_to_print.len()
    );
    pretty_print_many_spans(
        linker_files,
        spans_to_print
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, span)| (*span, format!("-{idx}: Span({}, {})", span.start, span.end))),
    );
}

/// Used by __debug_span
#[allow(unused)]
pub fn debug_print_span(span: Span, label: String) {
    let linker_ptr = STORED_LINKER.get();
    // SAFETY: Well actually this is totally not safe, since this could be called while a &mut Linker is held, and returned.
    // But since this is exclusively for debugging (and therefore should never be part of a release), it doesn't matter.
    if let Some(linker) = unsafe { linker_ptr.as_ref() } {
        pretty_print_span(&linker.files, span, label);
    } else {
        error!("DEBUG: No Linker registered for Span Debugging!");
    }
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

pub fn debug_context<R>(stage: &'static str, global_obj_name: String, f: impl FnOnce() -> R) -> R {
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

fn create_dump(linker: &Linker) {
    let config = crate::config();

    if config.no_redump {
        return;
    }

    use std::fs;
    use std::io::Write;

    // Get ./sus_crash_dumps/{timestamp}
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
            format!("{stage}_{global_obj_name}")
        } else {
            "unknown".to_string()
        }
    });

    let dump_name = sanitize_filename(&failure_name, &cur_time);
    let mut dump_dir = config.sus_home.join("crash_dumps").join(&dump_name);

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

            return;
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
    error!("Internal Compiler Error! All files dumped to {dump_dir:?}");
}

/// Set up the hook to print spans. Uses [std::panic::set_hook] instead of [std::panic::catch_unwind] because this runs before my debugger "on panic" breakpoint.
/// Use together with [create_dump_on_panic].
pub fn setup_panic_handler() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        default_hook(info);

        // === Display last touched spans ===

        // Additional barrier for errant optimizations?
        let linker_ptr: *mut Linker = std::hint::black_box(STORED_LINKER.get());
        if linker_ptr.is_null() {
            info!("sus_compiler crashed outside of an area protected by create_dump_on_panic");
            return;
        };
        // SAFETY: Haha, yeah this is technically UB, see [create_dump_on_panic]
        let linker = unsafe { &*linker_ptr };

        DEBUG_STACK.with_borrow(|history| {
            if let Some(last_stack_elem) = history.debug_stack.last() {
                eprintln!(
                    "{}",
                    format!(
                        "Panic happened in Span-guarded context {} in {}",
                        last_stack_elem.stage.red(),
                        last_stack_elem.global_obj_name.red()
                    )
                    .red()
                );

                //pretty_print_span(file_data, span, label);
                print_most_recent_spans(&linker.files, last_stack_elem);
            } else {
                eprintln!("{}", "No Span-guarding context".red());
            }
            eprintln!("{}", "Most recent available debug paths:".red());
            for (ctx, d) in &history.recent_debug_options {
                if let Some(ctx) = ctx {
                    eprintln!("{}", format!("--debug-whitelist {ctx} --debug {d}").red());
                } else {
                    eprintln!("{}", format!("(no SpanDebugger Context) --debug {d}").red());
                }
            }
        });

        // === Create Dump on Panic ===

        create_dump(linker);

        #[cfg(miri)]
        {
            // Miri can't handle deallocation of tree-sitter trees
            unsafe {
                std::mem::forget(std::mem::take(
                    &mut STORED_LINKER.get().as_mut().unwrap().files,
                ));
            }
        }
    }));
}

/// [setup_panic_handler] must be called before this
///
/// Okay so after lots of testing and thinking, I now know this *is* in fact Undefined Behavior.
/// The reason? The panic handler runs while the stack frame I'm debugging is still live.
///
/// Look, it's UB, but it's a better tradeoff than *not* having the debug info that the program can give me immediately upon panic.
/// STORED_LINKER is turned into a simple shared reference in [setup_panic_handler], which again should slightly limit the blast radius of this UB.
pub fn create_dump_on_panic<R>(linker: &mut Linker, f: impl FnOnce(&mut Linker) -> R) -> R {
    let linker_ptr: *mut Linker = linker as *mut Linker;
    let old_stored_linker = STORED_LINKER.replace(linker_ptr);
    if !old_stored_linker.is_null() {
        panic!("create_dump_on_panic is re-entrant? This is a logic error!");
    }

    // SAFETY: STORED_LINKER will only point to linker for the duration of the `f` function call. On a panic (and so when setup_panic_handler's handler runs),
    // the &mut Linker is no longer in scope, therefore STORED_LINKER is the only reference to it (disregarding the reborrow of `linker`).
    // The &mut passed to f must be derived from STORED_LINKER, passing linker directly would mean linker_ptr is an invalid reborrow
    // See https://users.rust-lang.org/t/how-to-report-extra-information-on-crashes-from-global-state-without-invalidating-a-pointer/136699
    let r = f(unsafe { &mut *linker_ptr });

    STORED_LINKER.replace(std::ptr::null_mut());
    //assert_eq!(STORED_LINKER.replace(None), Some(NonNull::from(linker)));

    r
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

#[cfg(test)]
mod tests {
    use std::cell::OnceCell;

    use crate::{errors::ErrorStore, file_position::FileText, linker::FileData};

    use super::*;

    /// Yeah I wish it were safe according to miri, but it is not. See [create_dump_on_panic]
    #[test]
    #[should_panic(expected = "OOPS")]
    fn test_miri_panic_handler() {
        crate::config::init_cfg_for_test();
        setup_panic_handler();
        let mut linker = Linker::new();

        create_dump_on_panic(&mut linker, |linker| {
            let fd_id = linker.files.alloc(FileData {
                file_identifier: "/non_existent/test/file/path.sus".to_string(),
                file_text: FileText::new("non extistent file text".to_string()),
                parsing_errors: ErrorStore::new(),
                associated_values: Vec::new(),
                tree: unsafe { tree_sitter::Tree::from_raw(std::ptr::dangling_mut()) },
                is_std: false,
                ariadne_source: OnceCell::new(),
            });

            debug_context("test_context", "test_obj".to_string(), || {
                let my_span = Span::from_range(4..13, fd_id); // "existent"
                my_span.debug();
                if !linker.files.is_empty() {
                    panic!("OOPS");
                }

                let linker_two = &*linker;

                println!("{}", linker_two.files[fd_id].is_std);
            });
        });
    }
}
