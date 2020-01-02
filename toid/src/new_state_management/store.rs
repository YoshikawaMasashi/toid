use std::marker::PhantomData;
use std::sync::Arc;

use super::reducer::Reducer;
use super::serialize::Serialize;

pub struct Store<S, E, R> {
    state: Arc<S>,
    reducer: Arc<R>,
    event_marker: PhantomData<E>,
}

impl<S, E: Sized + Serialize<E>, R: Reducer<S, E>> Store<S, E, R> {
    pub fn get_state(&self) -> Arc<S> {
        Arc::clone(&self.state)
    }

    pub fn update_state(&mut self, event: E) {
        let new_state = self.reducer.reduce(self.get_state(), event);
        self.state = Arc::new(new_state);
    }

    pub fn update_state_by_string(&mut self, event_string: String) {
        let event = E::deserialize(event_string).unwrap();
        self.update_state(event);
    }

    pub fn get_reducer(&self) -> Arc<R> {
        Arc::clone(&self.reducer)
    }
}
