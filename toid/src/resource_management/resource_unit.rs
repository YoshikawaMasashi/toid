use std::collections::BTreeMap;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

use serde_derive::Deserialize;
use toml;

use super::super::data::sf2::SF2;

#[derive(Deserialize)]
struct ResourceConfig {
    name: String,
    paths: BTreeMap<String, String>,
}

pub struct ResourceUnit {
    pub name: String,
    pub preference_path: Box<Path>,
    pub file_paths: BTreeMap<String, Box<Path>>,
    pub sf2: BTreeMap<String, Arc<SF2>>,
}

impl ResourceUnit {
    pub fn new(preference_path: String) -> ResourceUnit {
        let preference_toml = fs::read_to_string(preference_path.clone()).unwrap();
        let decoded_preference: ResourceConfig = toml::from_str(preference_toml.as_str()).unwrap();

        let mut file_paths: BTreeMap<String, Box<Path>> = BTreeMap::new();
        for (key, file) in decoded_preference.paths.iter() {
            file_paths.insert(
                key.clone(),
                Box::<Path>::from(Path::new(&preference_path).with_file_name(file)),
            );
        }

        ResourceUnit {
            name: decoded_preference.name,
            preference_path: Box::<Path>::from(Path::new(&preference_path)),
            file_paths,
            sf2: BTreeMap::new(),
        }
    }

    pub fn check_existance(&self) -> bool {
        let mut exist_all = true;
        for (_key, path) in self.file_paths.iter() {
            exist_all = exist_all && path.exists();
        }
        exist_all
    }

    pub fn load_sf2(&mut self, key: String) -> Result<(), String> {
        let path = &self.file_paths[&key];
        let mut f = fs::File::open(path).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();
        let buffer = buffer.as_slice();
        let sf2 = SF2::parse(buffer)?;
        let sf2 = Arc::new(sf2);
        self.sf2.insert(key, sf2);
        Ok(())
    }
}
