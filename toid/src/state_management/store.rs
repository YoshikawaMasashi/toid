/// Storeはstateを保持し、必要に応じてアップデートをする窓口を提供します。
pub struct Store<T> {
    state: T,
}

impl<T: Clone> Store<T> {
    pub fn new(initial_state: T) -> Self {
        Store {
            state: initial_state,
        }
    }

    pub fn update_state(&mut self, state: T) {
        self.state = state;
    }

    pub fn get_state(&self) -> T {
        return self.state.clone();
    }
}
