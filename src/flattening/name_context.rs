/// This keeps track of the variables that are in the current scope.
///
/// Each [super::Declaration] and [super::SubModuleInstance] should be added here at some point
///
/// Must be maintained manually.
/// When a new scope is entered, call [Self::new_frame],
/// when exiting a scope call [Self::pop_frame]
pub struct LocalVariableContext<'file, Obj: Copy> {
    local_stack: Vec<(&'file str, Obj)>,
    current_frame_starts_at: usize,
}

impl<'file, Obj: Copy> LocalVariableContext<'file, Obj> {
    pub fn get_declaration_for(&self, name: &'file str) -> Option<Obj> {
        for (decl_name, unique_id) in self.local_stack.iter().rev() {
            if *decl_name == name {
                return Some(*unique_id);
            }
        }
        None
    }
    pub fn add_declaration(
        &mut self,
        new_local_name: &'file str,
        new_local_unique_id: Obj,
    ) -> Result<(), Obj> {
        // Returns conflicting signal declaration
        for (existing_local_name, existing_local_id) in &self.local_stack {
            if new_local_name == *existing_local_name {
                return Err(*existing_local_id);
            }
        }
        self.local_stack.push((new_local_name, new_local_unique_id));
        Ok(())
    }
    pub fn new_initial() -> Self {
        Self {
            local_stack: Vec::new(),
            current_frame_starts_at: 0,
        }
    }
    pub fn new_frame(&mut self) -> usize {
        self.current_frame_starts_at = self.local_stack.len();
        self.current_frame_starts_at
    }
    pub fn pop_frame(&mut self, prev_save: usize) {
        assert!(self.current_frame_starts_at >= prev_save);
        self.current_frame_starts_at = prev_save;
        self.local_stack.truncate(prev_save);
    }
}
