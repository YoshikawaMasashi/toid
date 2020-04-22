use std::sync::Arc;

pub struct StateHolder<S> {
    state: Arc<S>,
}

impl<S> StateHolder<S> {
    pub fn new(state: S) -> Self {
        StateHolder {
            state: Arc::new(state),
        }
    }

    pub fn get_state(&self) -> Arc<S> {
        Arc::clone(&self.state)
    }

    pub fn set_state(&mut self, new_state: S) {
        self.state = Arc::new(new_state);
    }
}
