use std::{cell::RefCell, ops::Range};

use crate::{
    alloc::ArenaAllocator, config::ConfigStruct, flattening::Module, linker::FileData,
    pretty_print_spans_in_reverse_order, ModuleUUIDMarker,
};

/// Many duplicates will be produced, and filtering them out in the code itself is inefficient. Therefore just keep a big buffer and deduplicate as needed
const SPAN_TOUCH_HISTORY_SIZE: usize = 256;
const NUM_SPANS_TO_PRINT: usize = 10;
const DEFAULT_RANGE: Range<usize> = usize::MAX..usize::MAX;

/// Register a [crate::file_position::Span] for potential printing by [PanicGuardSpanPrinter] on panic.
///
/// Would like to use [crate::file_position::Span], but cannot copy the span because that would create infinite loop.
///
/// So use [Range] instead.
pub fn add_debug_span(span_rng: Range<usize>) {
    // Convert to range so we don't invoke any of Span's triggers
    SPANS_HISTORY.with_borrow_mut(|history| {
        let cur_idx = history.num_spans;
        history.num_spans += 1;

        history.span_history[cur_idx % SPAN_TOUCH_HISTORY_SIZE] = span_rng;
    });
}

struct TouchedSpansHistory {
    span_history: [Range<usize>; SPAN_TOUCH_HISTORY_SIZE],
    num_spans: usize,
    in_use: bool,
}

thread_local! {
    static SPANS_HISTORY : RefCell<TouchedSpansHistory> =
        const { RefCell::new(TouchedSpansHistory{
            span_history : [DEFAULT_RANGE; SPAN_TOUCH_HISTORY_SIZE],
            num_spans : 0,
            in_use : false
        }) };
}

fn print_most_recent_spans(file_data: &FileData) {
    let spans_to_print: Vec<Range<usize>> = SPANS_HISTORY.with_borrow_mut(|history| {
        assert!(history.in_use);

        let mut spans_to_print: Vec<Range<usize>> = Vec::with_capacity(NUM_SPANS_TO_PRINT);

        let end_at = if history.num_spans > SPAN_TOUCH_HISTORY_SIZE {
            history.num_spans - SPAN_TOUCH_HISTORY_SIZE
        } else {
            0
        };

        let mut cur_i = history.num_spans;
        while cur_i > end_at {
            cur_i -= 1;
            let grabbed_span = history.span_history[cur_i % SPAN_TOUCH_HISTORY_SIZE].clone();
            if !spans_to_print.contains(&grabbed_span) {
                spans_to_print.push(grabbed_span);
            }
            if spans_to_print.len() >= NUM_SPANS_TO_PRINT {
                break;
            }
        }

        spans_to_print
    });

    println!("Panic unwinding. Printing the last {} spans. BEWARE: These spans may not correspond to this file, thus incorrect spans are possible!", spans_to_print.len());
    pretty_print_spans_in_reverse_order(file_data, spans_to_print);
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
    context: &'text str,
    file_data: &'text FileData,
}

impl<'text> SpanDebugger<'text> {
    pub fn new(context: &'text str, file_text: &'text FileData) -> Self {
        SPANS_HISTORY.with_borrow_mut(|history| {
            assert!(!history.in_use);
            history.in_use = true;
            history.num_spans = 0;
        });

        Self {
            context,
            file_data: file_text,
        }
    }
}

impl Drop for SpanDebugger<'_> {
    fn drop(&mut self) {
        if std::thread::panicking() {
            println!("Panic happened in Span-guarded context: {}", self.context);
            print_most_recent_spans(self.file_data)
        }
        SPANS_HISTORY.with_borrow_mut(|history| {
            assert!(history.in_use);
            history.in_use = false;
        });
    }
}

impl ConfigStruct {
    /// The reason we pass an explicit bool here is because it merges the "if config().debug_xyz" with the for loop.
    pub fn for_each_debug_module<F: FnMut(&Module)>(
        &self,
        modules: &ArenaAllocator<Module, ModuleUUIDMarker>,
        mut f: F,
    ) {
        for (_, md) in modules {
            let passes_whitelist = if let Some(wl) = &self.debug_whitelist {
                wl.contains(&md.link_info.name)
            } else {
                true
            };
            if passes_whitelist {
                f(md)
            }
        }
    }

    pub fn should_print_for_debug(&self, md_name: &str) -> bool {
        if let Some(wl) = &self.debug_whitelist {
            wl.contains(md_name)
        } else {
            true
        }
    }
}
