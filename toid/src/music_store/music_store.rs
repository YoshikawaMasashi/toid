use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use super::super::state_management::store::Store;
use super::melody_state::{MelodyState, MelodyStateEvent, MelodyStateReducer};
use super::scheduling_state::{SchedulingState, SchedulingStateEvent, SchedulingStateReducer};
use super::sf2_state::{SF2State, SF2StateEvent, SF2StateReducer};

pub struct MusicStore {
    pub scheduling: Store<SchedulingState, SchedulingStateEvent, SchedulingStateReducer>,
    pub melody: RwLock<HashMap<String, Arc<Store<MelodyState, MelodyStateEvent, MelodyStateReducer>>>>,
    pub sf2: Store<SF2State, SF2StateEvent, SF2StateReducer>,
}

impl MusicStore {
    pub fn new() -> Self {
        MusicStore {
            scheduling: Store::new(SchedulingState::new(), SchedulingStateReducer {}),
            melody: RwLock::new(HashMap::new()),
            sf2: Store::new(SF2State::new(), SF2StateReducer {}),
        }
    }

    pub fn new_melody(&self, key: String) {
        self.melody
            .write()
            .unwrap()
            .insert(key, Arc::new(Store::new(MelodyState::new(), MelodyStateReducer {})));
    }
}
