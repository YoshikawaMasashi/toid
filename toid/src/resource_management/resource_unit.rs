use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde_derive::Deserialize;
use toml;

#[derive(Deserialize)]
struct ResourceConfig {
    name: String,
    paths: BTreeMap<String, String>,
}

pub struct ResourceUnit {
    name: String,
    preference_path: Box<Path>,
    file_paths: BTreeMap<String, Box<Path>>,
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
        }
    }

    pub fn check_existance(&self) -> bool {
        let mut exist_all = true;
        for (key, path) in self.file_paths.iter() {
            exist_all = exist_all && path.exists();
        }
        exist_all
    }
}
