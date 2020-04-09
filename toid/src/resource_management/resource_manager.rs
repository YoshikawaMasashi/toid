use std::collections::BTreeMap;
use std::path::Path;

use super::resource_unit::ResourceUnit;

pub struct ResourceManager {
    units: BTreeMap<String, ResourceUnit>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            units: BTreeMap::new(),
        }
    }

    pub fn register(&mut self, path: String) {
        let new_unit = ResourceUnit::new(path);
        if new_unit.check_existance() {
            self.units.insert(new_unit.name.clone(), new_unit);
        } else {
            panic!("check existance error!")
        }
    }

    pub fn get_path(&self, name: String) -> Result<Box<Path>, String> {
        if let Some(dot_idx) = name.find('.') {
            let (first, last) = name.split_at(dot_idx);
            let unit = &self.units[first];
            Ok(unit.file_paths[last.split_at(1).1].clone())
        } else {
            Err(String::from("invalid name"))
        }
    }
}
