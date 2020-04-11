use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::RwLock;

use super::super::state_management::reducer::Reducer;
use super::super::state_management::serialize::Serialize;
use super::super::state_management::state_holder::StateHolder;
use super::super::state_management::store::Store;

pub struct NormalStore<S, E, R> {
    state_holder: RwLock<StateHolder<S>>,
    reducer: Arc<R>,
    event_marker: PhantomData<E>,
}

impl<S, E: Sized + Serialize<E>, R: Reducer<S, E>> NormalStore<S, E, R> {
    pub fn new(state: S, reducer: R) -> Self {
        Self {
            state_holder: RwLock::new(StateHolder::new(state)),
            reducer: Arc::new(reducer),
            event_marker: PhantomData,
        }
    }
}

impl<S, E: Sized + Serialize<E>, R: Reducer<S, E>> Store<S, E> for NormalStore<S, E, R> {
    fn get_state(&self) -> Arc<S> {
        Arc::clone(&self.state_holder.read().unwrap().get_state())
    }

    fn update_state(&self, event: E) {
        let new_state = self.reducer.reduce(self.get_state(), event);
        self.state_holder.write().unwrap().set_state(new_state);
    }
}
