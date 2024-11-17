

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DelayedConstraintStatus {
    Resolved,
    Progress,
    NoProgress
}

pub trait DelayedConstraint<T> {
    fn try_apply(&mut self, shared_object : &mut T) -> DelayedConstraintStatus;
    fn report_could_not_resolve_error(&self, shared_object : &T);
}

/// This is for unification of constraints that may not be resolveable right away
/// 
/// Such as struct field access. vec.x cannot resolve the type of x before the type of vec has been resolved
/// 
/// The given function should only make changes when it can be successfully resolved
/// 
/// When the constraint has been resolved, it should return 'true'
/// 
/// For convenience, a &mut T is provided such that a shared mutable object can be used
pub struct DelayedConstraintsList<T>(Vec<Box<dyn DelayedConstraint<T>>>);

impl<T> DelayedConstraintsList<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Add a constraint
    pub fn push<C : DelayedConstraint<T> + 'static>(&mut self, constraint: C) {
        self.0.push(Box::new(constraint));
    }

    /// Will keep looping over the list of constraints, and try to apply them. 
    /// 
    /// Calls [DelayedConstraint::report_could_not_resolve_error] on all constraints that weren't resolved
    pub fn resolve_delayed_constraints(mut self, shared_object: &mut T) {
        while self.0.len() > 0 {
            let mut progress_made = false;
            self.0.retain_mut(|constraint| {
                match constraint.try_apply(shared_object) {
                    DelayedConstraintStatus::Resolved => {progress_made = true; false}
                    DelayedConstraintStatus::Progress => {progress_made = true; true}
                    DelayedConstraintStatus::NoProgress => true
                }
            });
            if !progress_made {
                for constraint in std::mem::replace(&mut self.0, Vec::new()) {
                    constraint.report_could_not_resolve_error(shared_object);
                }
                return; // Exit
            }
        }
    }
}

impl<T> Drop for DelayedConstraintsList<T> {
    fn drop(&mut self) {
        assert_eq!(self.0.len(), 0, "DelayedConstraintsList was not resolved. ");
    }
}
