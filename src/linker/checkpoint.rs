use crate::errors::ErrorCollector;

use super::{LinkInfo, ResolvedGlobals};

#[derive(Debug,Clone,Copy)]
pub struct ErrorCheckpoint(pub usize, pub bool);

#[derive(Debug,Clone,Copy)]
pub struct ResolvedGlobalsCheckpoint(pub usize, pub bool);


#[derive(Debug,Clone,Copy)]
pub struct CheckPoint {
    errors_cp : ErrorCheckpoint,
    resolved_globals_cp : ResolvedGlobalsCheckpoint
}

impl CheckPoint {
    pub fn checkpoint(errors : &ErrorCollector, resolved_globals : &ResolvedGlobals) -> CheckPoint {
        CheckPoint {
            errors_cp : errors.checkpoint(),
            resolved_globals_cp : resolved_globals.checkpoint()
        }
    }
}

impl LinkInfo {
    pub fn reset_to(&mut self, cp : CheckPoint) {
        self.errors.reset_to(cp.errors_cp);
        self.resolved_globals.reset_to(cp.resolved_globals_cp);
    }
}
