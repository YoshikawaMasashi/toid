use std::fs;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

use serde_derive::Deserialize;
use toml;

use super::super::super::data::sf2::SF2;
use super::ResourceUnit;

#[derive(Deserialize)]
struct SF2Config {
    resourcetype: String,
    name: String,
    path: String,
}

pub struct SF2ResourceUnit {
    pub name: String,
    pub config_path: Box<Path>,
    pub file_path: Box<Path>,
    pub sf2: Arc<SF2>,
}

impl ResourceUnit for SF2ResourceUnit {
    fn load_toml(path: String) -> Result<Self, String> {
        let config_toml = fs::read_to_string(path.clone()).map_err(|_| "read error")?;
        let decoded_config: SF2Config =
            toml::from_str(config_toml.as_str()).map_err(|e| e.to_string())?;

        if decoded_config.resourcetype != "sf2" {
            return Err("is not sf2".to_string());
        }

        let file_path = Path::new(&path).with_file_name(decoded_config.path);
        let mut f = fs::File::open(file_path.clone()).map_err(|_| "file open error")?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).map_err(|_| "read error")?;
        let buffer = buffer.as_slice();
        let sf2 = SF2::parse(buffer)?;
        let sf2 = Arc::new(sf2);

        Ok(SF2ResourceUnit {
            name: decoded_config.name,
            config_path: Box::<Path>::from(Path::new(&path)),
            file_path: Box::<Path>::from(file_path),
            sf2,
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
        SF2ResourceUnit::load_toml("toid-sample-resource/sf2/sf2.toml".to_string()).unwrap();
    }
}
