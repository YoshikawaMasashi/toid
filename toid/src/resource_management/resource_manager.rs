use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;

use super::resource_unit::ResourceUnit;

pub struct ResourceManager {
    units: Arc<RwLock<BTreeMap<String, ResourceUnit>>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            units: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    pub fn register(&self, path: String) {
        let new_unit = ResourceUnit::new(path);
        if new_unit.check_existance() {
            self.units
                .write()
                .unwrap()
                .insert(new_unit.name.clone(), new_unit);
        } else {
            panic!("check existance error!")
        }
    }

    pub fn get_path(&self, name: String) -> Result<Box<Path>, String> {
        if let Some(dot_idx) = name.find('.') {
            let (first, last) = name.split_at(dot_idx);
            let last = last.split_at(1).1;
            Ok(self
                .units
                .read()
                .unwrap()
                .get(first)
                .unwrap()
                .file_paths
                .get(last)
                .unwrap()
                .clone())
        } else {
            Err(String::from("invalid name"))
        }
    }

    pub fn load_sf2(&self, name: String) {
        if let Some(dot_idx) = name.find('.') {
            let (first, last) = name.split_at(dot_idx);
            let last = last.split_at(1).1;
            self.units
                .write()
                .unwrap()
                .get_mut(first)
                .unwrap()
                .load_sf2(last.to_string());
        }
    }
}
