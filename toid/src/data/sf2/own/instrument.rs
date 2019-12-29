use std::sync::Arc;

use super::generator::Generator;

pub struct Instrument {
    name: String,
    generators: Vec<Arc<Generator>>,
}

impl Instrument {
    pub fn new() -> Self {
        Instrument {
            name: String::from(""),
            generators: Vec::new(),
        }
    }

    pub fn add_generator(&mut self, generator: Arc<Generator>) {
        self.generators.push(generator);
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
