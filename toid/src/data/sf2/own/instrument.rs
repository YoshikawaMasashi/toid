use std::sync::Arc;

use super::generator::InstrumentGenerator;

pub struct Instrument {
    name: String,
    generators: Vec<Arc<InstrumentGenerator>>,
}

impl Instrument {
    pub fn new() -> Self {
        Instrument {
            name: String::from(""),
            generators: Vec::new(),
        }
    }

    pub fn add_generator(&mut self, generator: Arc<InstrumentGenerator>) {
        self.generators.push(generator);
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
