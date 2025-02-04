use crate::errors::ErrorStore;

use super::{LinkInfo, ResolvedGlobals};

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
        assert!(checkpoint_id < self.checkpoints.len());
        let cp = self.checkpoints[checkpoint_id];
        self.checkpoints.truncate(checkpoint_id + 1);
        self.errors.reset_to(cp.errors_cp);
        self.resolved_globals.reset_to(cp.resolved_globals_cp);
    }
}
