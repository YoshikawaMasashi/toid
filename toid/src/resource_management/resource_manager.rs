use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;

use serde::{Deserialize, Serialize};

use super::super::data::sf2::SF2;
use super::super::state_management::serialize;
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

    pub fn register(&self, path: String) -> Result<(), String> {
        let new_unit = ResourceUnit::new(path)?;
        if new_unit.check_existance() {
            self.units
                .write()
                .map_err(|_| "RwLock Error")?
                .insert(new_unit.name.clone(), new_unit);
            Ok(())
        } else {
            Err("check existance error!".to_string())
        }
    }

    pub fn get_path(&self, name: String) -> Result<Box<Path>, String> {
        if let Some(dot_idx) = name.find('.') {
            let (first, last) = name.split_at(dot_idx);
            let last = last.split_at(1).1;
            Ok(self
                .units
                .read()
                .map_err(|_| "RwLock Error")?
                .get(first)
                .ok_or("get Error")?
                .file_paths
                .get(last)
                .ok_or("get Error")?
                .clone())
        } else {
            Err(String::from("invalid name"))
        }
    }

    fn load_sf2(&self, name: String) -> Result<(), String> {
        if let Some(dot_idx) = name.find('.') {
            let (first, last) = name.split_at(dot_idx);
            let last = last.split_at(1).1;
            self.units
                .write()
                .map_err(|_| "RwLock Error")?
                .get_mut(first)
                .ok_or("get Error")?
                .load_sf2(last.to_string())?;
            Ok(())
        } else {
            Err(format!("invalid name {}", name).to_string())
        }
    }

    pub fn get_sf2(&self, name: String) -> Result<Arc<SF2>, String> {
        if let Some(dot_idx) = name.find('.') {
            let (first, last) = name.split_at(dot_idx);
            let last = last.split_at(1).1;
            let sf2 = Arc::clone(
                self.units
                    .read()
                    .map_err(|_| "RwLock Error")?
                    .get(first)
                    .ok_or("get Error")?
                    .sf2
                    .get(last)
                    .ok_or("get Error")?,
            );
            Ok(sf2)
        } else {
            Err(String::from("invalid name"))
        }
    }

    pub fn apply(&self, event: ResourceManagerEvent) -> Result<(), String> {
        match event {
            ResourceManagerEvent::LoadSF2(name) => self.load_sf2(name)?,
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub enum ResourceManagerEvent {
    LoadSF2(String),
}

impl serialize::Serialize<ResourceManagerEvent> for ResourceManagerEvent {
    fn serialize(&self) -> Result<String, String> {
        match serde_json::to_string(&self) {
            Ok(serialized) => Ok(serialized),
            Err(err) => Err(format!("error in serizalization : {}", err)),
        }
    }
    fn deserialize(serialized: String) -> Result<Self, String> {
        match serde_json::from_str(serialized.as_str()) {
            Ok(deserialized) => Ok(deserialized),
            Err(err) => Err(format!("error in deserizalization : {}", err)),
        }
    }
}
