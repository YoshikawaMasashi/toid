use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::RwLock;

use super::super::state_management::serialize::Serialize;
use super::super::state_management::state::State;
use super::super::state_management::state_holder::StateHolder;
use super::super::state_management::store::Store;

pub struct NormalStore<S, E> {
    state_holder: RwLock<StateHolder<S>>,
    event_marker: PhantomData<E>,
}

impl<S, E> NormalStore<S, E> {
    pub fn new(state: S) -> Self {
        Self {
            state_holder: RwLock::new(StateHolder::new(state)),
            event_marker: PhantomData,
        }
    }
}

impl<S: State<E>, E: Sized + Serialize<E>> Store<S, E> for NormalStore<S, E> {
    fn get_state(&self) -> Arc<S> {
        Arc::clone(&self.state_holder.read().unwrap().get_state())
    }

    fn update_state(&self, event: E) {
        let new_state = self.get_state().reduce(event);
        self.state_holder.write().unwrap().set_state(new_state);
    }
}
