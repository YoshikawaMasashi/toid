use std::sync::Arc;

use super::generator::Generator;

pub struct Instrument {
    generators: Vec<Arc<Generator>>,
}

impl Instrument {
    pub fn new() -> Self {
        Instrument {
            generators: Vec::new(),
        }
    }

    pub fn add_generator(&mut self, generator: Arc<Generator>) {
        self.generators.push(generator);
    }
}
