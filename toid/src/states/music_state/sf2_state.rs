use std::sync::Arc;

use super::super::super::data::sf2::SF2;

pub struct SF2State {
    pub sf2: Option<Arc<SF2>>,
}

impl Clone for SF2State {
    fn clone(&self) -> Self {
        SF2State {
            sf2: match &self.sf2 {
                Some(sf2) => Some(Arc::clone(&sf2)),
                None => None,
            },
        }
    }
}

impl SF2State {
    pub fn new() -> Self {
        SF2State { sf2: None }
    }

    pub fn set_sf2(&self, sf2: Arc<SF2>) -> Self {
        SF2State {
            sf2: Some(Arc::clone(&sf2)),
        }
    }
}
