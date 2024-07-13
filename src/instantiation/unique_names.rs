use std::collections::HashMap;


pub struct UniqueNames {
    name_map : HashMap<String, i64>
}

impl UniqueNames {
    pub fn new() -> Self {
        let mut name_map : HashMap<String, i64> = HashMap::new();
        name_map.insert(String::new(), 1);
        Self {
            name_map
        }
    }
    pub fn get_unique_name<S : Into<String> + AsRef<str>>(&mut self, name : S) -> String {
        let name_ref = name.as_ref();
        if let Some(found_id) = self.name_map.get_mut(name_ref) {
            let result = format!("{name_ref}_{found_id}");
            *found_id += 1;
            result
        } else {
            let result : String = name.into();
            self.name_map.insert(result.clone(), 2);
            result
        }
    }
}
