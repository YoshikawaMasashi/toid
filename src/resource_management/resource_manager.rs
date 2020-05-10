use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::RwLock;

use serde::{Deserialize, Serialize};

use super::super::data::sf2::SF2;
use super::super::data::wave::Wave;
use super::super::state_management::serialize;
use super::resource_units::ResourceUnitEnum;

pub struct ResourceManager {
    units: Arc<RwLock<BTreeMap<String, ResourceUnitEnum>>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            units: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    pub fn register(&self, path: String) -> Result<(), String> {
        let new_unit = ResourceUnitEnum::load_toml(path)?;
        self.units
            .write()
            .map_err(|_| "RwLock Error")?
            .insert(new_unit.get_name().clone(), new_unit);
        Ok(())
    }

    pub fn get_sf2(&self, name: String) -> Result<Arc<SF2>, String> {
        match self
            .units
            .read()
            .map_err(|_| "RwLock Error")?
            .get(&name)
            .ok_or("get Error")?
        {
            ResourceUnitEnum::SF2(sf2) => Ok(Arc::clone(&sf2.sf2)),
            _ => Err("this name is not sf2".to_string()),
        }
    }

    pub fn get_drums_wave(&self, name: String, sound: String) -> Result<Arc<Wave>, String> {
        match self
            .units
            .read()
            .map_err(|_| "RwLock Error")?
            .get(&name)
            .ok_or("get Error")?
        {
            ResourceUnitEnum::Drums(drums) => match drums.waves.get(&sound) {
                Some(wave) => Ok(Arc::clone(&wave)),
                None => Err("there is not wave of sound string".to_string()),
            },
            _ => Err("this name is not sf2".to_string()),
        }
    }

    pub fn apply(&self, _: ResourceManagerEvent) -> Result<(), String> {
        // match event {}
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub enum ResourceManagerEvent {}

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
