use super::super::state_management::store::Store;
use super::melody_state::{MelodyState, MelodyStateEvent, MelodyStateReducer};
use super::scheduling_state::{SchedulingState, SchedulingStateEvent, SchedulingStateReducer};
use super::sf2_state::{SF2State, SF2StateEvent, SF2StateReducer};

pub struct NewMusicStore {
    pub scheduling: Store<SchedulingState, SchedulingStateEvent, SchedulingStateReducer>,
    pub melody: Store<MelodyState, MelodyStateEvent, MelodyStateReducer>,
    pub sf2: Store<SF2State, SF2StateEvent, SF2StateReducer>,
}

impl NewMusicStore {
    pub fn new() -> Self {
        NewMusicStore {
            scheduling: Store::new(SchedulingState::new(), SchedulingStateReducer {}),
            melody: Store::new(MelodyState::new(), MelodyStateReducer {}),
            sf2: Store::new(SF2State::new(), SF2StateReducer {}),
        }
    }
}
