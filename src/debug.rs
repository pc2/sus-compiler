use std::{cell::RefCell, collections::HashSet, ops::Range, sync::OnceLock};

use circular_buffer::CircularBuffer;

use crate::{config::config, linker::FileData, prelude::Span, pretty_print_spans_in_reverse_order};

/// Many duplicates will be produced, and filtering them out in the code itself is inefficient. Therefore just keep a big buffer and deduplicate as needed
const SPAN_TOUCH_HISTORY_SIZE: usize = 256;
const NUM_SPANS_TO_PRINT: usize = 10;

struct SpanDebuggerStackElement {
    context: String,
    debugging_enabled: bool,
    span_history: CircularBuffer<SPAN_TOUCH_HISTORY_SIZE, Span>,
}

thread_local! {
    static DEBUG_STACK : RefCell<Vec<SpanDebuggerStackElement>> = const { RefCell::new(Vec::new()) };
}

/// Register a [crate::file_position::Span] for potential printing by [PanicGuardSpanPrinter] on panic.
///
/// Would like to use [crate::file_position::Span], but cannot copy the span because that would create infinite loop.
///
/// So use [Range] instead.
pub fn add_debug_span(sp: Span) {
    // Convert to range so we don't invoke any of Span's triggers
    DEBUG_STACK.with_borrow_mut(|history| {
        let Some(last) = history.last_mut() else {
            return; // Can't track Spans not in a SpanDebugger region
        };

        last.span_history.push_back(sp);
    });
}

fn print_most_recent_spans(file_data: &FileData) {
    DEBUG_STACK.with_borrow_mut(|history| {
        let history = history.last_mut().unwrap();

        let mut spans_to_print: Vec<Range<usize>> = Vec::with_capacity(NUM_SPANS_TO_PRINT);

        for sp in history.span_history.iter().rev() {
            let as_range = sp.as_range();
            if !spans_to_print.contains(&as_range) {
                spans_to_print.push(as_range);
            }
            if spans_to_print.len() >= NUM_SPANS_TO_PRINT {break;}
        }

        println!("Panic unwinding. Printing the last {} spans. BEWARE: These spans may not correspond to this file, thus incorrect spans are possible!", spans_to_print.len());
        pretty_print_spans_in_reverse_order(file_data, spans_to_print);
    });
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
}

fn print_stack_top(enter_exit: &str) {
    DEBUG_STACK.with_borrow(|stack| {
        if let Some(top) = stack.last() {
            let debug_enabled = if top.debugging_enabled {
                " DEBUGGING ENABLED"
            } else {
                ""
            };
            println!(
                "{enter_exit}SpanDebugger (x{}) {}{debug_enabled}",
                stack.len(),
                top.context
            );
        } else {
            println!("SpanDebugger (x0)")
        }
    })
}

impl<'text> SpanDebugger<'text> {
    pub fn new(stage: &'static str, global_obj_name: &str, file_data: &'text FileData) -> Self {
        let context = format!("{stage} {global_obj_name}");
        DEBUG_STACK.with_borrow_mut(|history| {
            let config = config();

            let debugging_enabled = config.debug_whitelist.is_empty()
                && !ENABLED_DEBUG_PATHS.get().unwrap().is_empty()
                || config
                    .debug_whitelist
                    .iter()
                    .any(|v| global_obj_name.contains(v));

            history.push(SpanDebuggerStackElement {
                context,
                debugging_enabled,
                span_history: CircularBuffer::new(),
            });
        });
        print_stack_top("Enter ");

        Self { file_data }
    }
}

impl Drop for SpanDebugger<'_> {
    fn drop(&mut self) {
        print_stack_top("Exit ");
        DEBUG_STACK.with_borrow_mut(|stack| stack.pop());
        print_stack_top("");

        if std::thread::panicking() {
            eprintln!("Panic happened in Span-guarded context");
            print_most_recent_spans(self.file_data)
        }
    }
}

pub static ENABLED_DEBUG_PATHS: OnceLock<HashSet<String>> = OnceLock::new();

/// Check if the debug path is enabled
pub fn is_enabled(path_id: &'static str) -> bool {
    DEBUG_STACK.with_borrow(|stack| {
        let Some(last) = stack.last() else {
            return false;
        };
        if !last.debugging_enabled {
            return false;
        }
        let Some(debug_paths) = ENABLED_DEBUG_PATHS.get() else {
            return false;
        };

        debug_paths.contains(path_id)
    })
}
