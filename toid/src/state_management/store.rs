use std::sync::Arc;

/// Storeはstateを保持し、必要に応じてアップデートをする窓口を提供します。
pub struct Store<T> {
    state: Arc<T>,
}

impl<T> Store<T> {
    pub fn new(initial_state: T) -> Self {
        let state = Arc::new(initial_state);
        Store { state }
    }

    pub fn update_state(&mut self, state: T) {
        let new_state = Arc::new(state);
        self.state = new_state;
    }

    pub fn get_state(&self) -> Arc<T> {
        return Arc::clone(&self.state);
    }
}
