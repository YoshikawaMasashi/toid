use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::RwLock;

use super::reducer::Reducer;
use super::serialize::Serialize;

pub struct StateHolder<S> {
    state: Arc<S>,
}

impl<S> StateHolder<S> {
    fn new(state: S) -> Self {
        StateHolder {
            state: Arc::new(state),
        }
    }

    fn get_state(&self) -> Arc<S> {
        Arc::clone(&self.state)
    }

    fn set_state(&mut self, new_state: S) {
        self.state = Arc::new(new_state);
    }
}

pub struct Store<S, E, R> {
    state_holder: RwLock<StateHolder<S>>,
    reducer: Arc<R>,
    event_marker: PhantomData<E>,
}

impl<S, E: Sized + Serialize<E>, R: Reducer<S, E>> Store<S, E, R> {
    pub fn new(state: S, reducer: R) -> Self {
        Self {
            state_holder: RwLock::new(StateHolder::new(state)),
            reducer: Arc::new(reducer),
            event_marker: PhantomData,
        }
    }

    pub fn get_state(&self) -> Arc<S> {
        Arc::clone(&self.state_holder.read().unwrap().get_state())
    }

    pub fn update_state(&self, event: E) {
        let new_state = self.reducer.reduce(self.get_state(), event);
        self.state_holder.write().unwrap().set_state(new_state);
    }

    pub fn update_state_by_string(&mut self, event_string: String) {
        let event = E::deserialize(event_string).unwrap();
        self.update_state(event);
    }

    pub fn get_reducer(&self) -> Arc<R> {
        Arc::clone(&self.reducer)
    }
}
