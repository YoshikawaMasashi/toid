use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

use serde_derive::Deserialize;
use toml;

use super::super::super::data::wave::Wave;
use super::ResourceUnit;

#[derive(Deserialize)]
struct SamplesConfig {
    resourcetype: String,
    name: String,
    waves: HashMap<String, String>,
}

pub struct SamplesResourceUnit {
    pub name: String,
    pub config_path: Box<Path>,
    pub file_paths: HashMap<String, Box<Path>>,
    pub waves: HashMap<String, Arc<Wave>>,
}

impl ResourceUnit for SamplesResourceUnit {
    fn load_toml(path: String) -> Result<Self, String> {
        let config_toml = fs::read_to_string(path.clone()).map_err(|_| "read error")?;
        let decoded_config: SamplesConfig =
            toml::from_str(config_toml.as_str()).map_err(|e| e.to_string())?;

        if decoded_config.resourcetype != "samples" {
            return Err("is not samples".to_string());
        }

        let mut file_paths = HashMap::new();
        let mut waves = HashMap::new();
        for (key, value) in decoded_config.waves.iter() {
            let file_path = Path::new(&path).with_file_name(value);
            file_paths.insert(key.clone(), Box::<Path>::from(file_path.clone()));

            let mut f = fs::File::open(file_path.clone()).map_err(|_| "file open error")?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).map_err(|_| "read error")?;
            let buffer = buffer.as_slice();
            let wave = Wave::parse(buffer)?;
            let wave = Arc::new(wave);
            waves.insert(key.clone(), wave);
        }

        Ok(SamplesResourceUnit {
            name: decoded_config.name,
            config_path: Box::<Path>::from(Path::new(&path)),
            file_paths,
            waves,
        })
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        SamplesResourceUnit::load_toml("toid-sample-resource/samples/samples.toml".to_string())
            .unwrap();
    }
}
