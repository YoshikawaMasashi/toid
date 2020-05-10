pub mod sf2;

use std::fs;

use serde_derive::Deserialize;
use toml;

use sf2::SF2ResourceUnit;

#[derive(Deserialize)]
struct ResourceConfig {
    pub resourcetype: String,
}

pub trait ResourceUnit
where
    Self: Sized,
{
    fn load_toml(path: String) -> Result<Self, String>;
    fn get_name(&self) -> String;
}

pub enum ResourceUnitEnum {
    SF2(SF2ResourceUnit),
}

impl ResourceUnitEnum {
    pub fn load_toml(path: String) -> Result<Self, String> {
        let config_toml = fs::read_to_string(path.clone()).map_err(|_| "read error")?;
        let decoded_config: ResourceConfig =
            toml::from_str(config_toml.as_str()).map_err(|e| e.to_string())?;

        match decoded_config.resourcetype.as_str() {
            "sf2" => Ok(ResourceUnitEnum::SF2(
                SF2ResourceUnit::load_toml(path).unwrap(),
            )),
            _ => Err("invalid resource_type".to_string()),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            ResourceUnitEnum::SF2(sf2) => sf2.get_name(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        ResourceUnitEnum::load_toml("toid-sample-resource/sf2/sf2.toml".to_string()).unwrap();
    }
}
