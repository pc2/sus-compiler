

pub struct LocalVariableContext<'prev, 'file, IdT: Copy> {
    locals : Vec<(&'file str, IdT)>,
    outer : Option<&'prev LocalVariableContext<'prev, 'file, IdT>>
}

impl<'prev, 'file, IdT: Copy> LocalVariableContext<'prev, 'file, IdT> {
    pub fn get_declaration_for(&self, name : &'file str) -> Option<IdT> {
        for (decl_name, unique_id) in &self.locals {
            if *decl_name == name {
                return Some(*unique_id);
            }
        }
        if let Some(p) = self.outer {
            p.get_declaration_for(name)
        } else {
            None
        }
    }
    pub fn add_declaration(&mut self, new_local_name : &'file str, new_local_unique_id : IdT) -> Result<(), IdT> { // Returns conflicting signal declaration
        for (existing_local_name, existing_local_id) in &self.locals {
            if new_local_name == *existing_local_name {
                return Err(*existing_local_id)
            }
        }
        self.locals.push((new_local_name, new_local_unique_id));
        Ok(())
    }
    pub fn new_initial() -> Self {
        Self{locals : Vec::new(), outer : None}
    }
    pub fn extend(&'prev self) -> LocalVariableContext<'prev, 'file, IdT> {
        LocalVariableContext{locals : Vec::new(), outer : Some(self)}
    }
}

