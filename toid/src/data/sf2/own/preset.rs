use std::sync::Arc;

use super::generator::Generator;

pub struct Preset {
    generators: Vec<Arc<Generator>>,
}

impl Preset {
    pub fn new() -> Self {
        Preset {
            generators: Vec::new(),
        }
    }

    pub fn add_generator(&mut self, generator: Arc<Generator>) {
        self.generators.push(generator);
    }
}
