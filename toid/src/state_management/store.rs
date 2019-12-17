use std::sync::Arc;

use super::state::State;

/// Storeはstateを保持し、必要に応じてアップデートをする窓口を提供します。
pub struct Store {
    state: State,
}

impl Store {
    pub fn new(initial_state: State) -> Self {
        Store {
            state: initial_state,
        }
    }

    pub fn update_state(&mut self, state: State) {
        self.state = state;
    }

    pub fn get_state(&self) -> State {
        return self.state.clone();
    }
}
