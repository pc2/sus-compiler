use crate::{
    errors::ErrorStore,
    linker::{GlobalUUID, Linker},
};

use super::{LinkInfo, ResolvedGlobals};

pub const AFTER_INITIAL_PARSE_CP: usize = 0;
pub const AFTER_FLATTEN_CP: usize = 1;
pub const AFTER_TYPE_CHECK_CP: usize = 2;
pub const AFTER_LINTS_CP: usize = 3;

const CHECKPOINT_NAMES: [&str; 4] = [
    "AFTER_INITIAL_PARSE_CP",
    "AFTER_FLATTEN_CP",
    "AFTER_TYPE_CHECK_CP",
    "AFTER_LINTS_CP",
];

/// Checkpoints [LinkInfo::errors]
///
/// For incremental builds (#49)
#[derive(Debug, Clone, Copy)]
pub struct ErrorCheckpoint(pub usize, pub bool);

/// Checkpoints [LinkInfo::resolved_globals]
///
/// For incremental builds (#49)
#[derive(Debug, Clone, Copy)]
pub struct ResolvedGlobalsCheckpoint(pub usize, pub bool);

/// See [LinkInfo::checkpoints]
///
/// For incremental builds (#49)
#[derive(Debug, Clone, Copy)]
pub struct CheckPoint {
    errors_cp: ErrorCheckpoint,
    resolved_globals_cp: ResolvedGlobalsCheckpoint,
}

impl CheckPoint {
    pub fn new(errors: &ErrorStore, resolved_globals: &ResolvedGlobals) -> CheckPoint {
        CheckPoint {
            errors_cp: errors.checkpoint(),
            resolved_globals_cp: resolved_globals.checkpoint(),
        }
    }
}

impl LinkInfo {
    pub fn reset_to(&mut self, checkpoint_id: usize) {
        assert!(
            checkpoint_id < self.checkpoints.len(),
            "Checkpoint in {}. Selected checkpoint is {} but the last checkpoint is {} which comes before",
            self.get_full_name(),
            CHECKPOINT_NAMES[checkpoint_id],
            CHECKPOINT_NAMES[self.checkpoints.len() - 1],
        );
        self.checkpoints.truncate(checkpoint_id + 1);
        let cp = self.checkpoints[checkpoint_id];
        self.errors.reset_to(cp.errors_cp);
        self.resolved_globals.reset_to(cp.resolved_globals_cp);
    }
}

impl Linker {
    pub fn checkpoint(&mut self, global_ids: &[GlobalUUID], checkpoint_id: usize) {
        for id in global_ids {
            let link_info = &mut self.globals[*id];

            let expected_checkpoint = link_info.checkpoints.len();
            assert!(expected_checkpoint == checkpoint_id,
                "In {}: The new checkpoint is not what was expected. The new checkpoint was {}, whereas the expected next checkpoint is {}",
                link_info.get_full_name(),
                CHECKPOINT_NAMES[checkpoint_id],
                CHECKPOINT_NAMES[expected_checkpoint],
            );

            link_info.checkpoints.push(CheckPoint::new(
                &link_info.errors,
                &link_info.resolved_globals,
            ));
        }
    }
}
