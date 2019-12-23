use super::super::state_management::store::Store;

pub struct DefaultStore<T: Clone> {
    state: T,
}

impl<T: Clone> DefaultStore<T> {
    pub fn new(initial_state: T) -> Self {
        DefaultStore {
            state: initial_state,
        }
    }
}

impl<T: Clone> Store<T> for DefaultStore<T> {
    fn update_state(&mut self, state: T) {
        self.state = state;
    }

    fn get_state(&self) -> T {
        return self.state.clone();
    }
}
