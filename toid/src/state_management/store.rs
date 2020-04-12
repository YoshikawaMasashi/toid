/*
use std::sync::Arc;

pub trait Store<S, E> {
    fn get_state(&self) -> Arc<S>;
    fn update_state(&self, event: E);
}
*/

use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::RwLock;

use super::serialize::Serialize;
use super::state::State;
use super::state_holder::StateHolder;

pub struct Store<S, E> {
    state_holder: RwLock<StateHolder<S>>,
    event_marker: PhantomData<E>,
}

impl<S: State<E>, E: Sized + Serialize<E>> Store<S, E> {
    pub fn new(state: S) -> Self {
        Self {
            state_holder: RwLock::new(StateHolder::new(state)),
            event_marker: PhantomData,
        }
    }

    pub fn get_state(&self) -> Arc<S> {
        Arc::clone(&self.state_holder.read().unwrap().get_state())
    }

    pub fn update_state(&self, event: E) {
        let new_state = self.get_state().reduce(event);
        self.state_holder.write().unwrap().set_state(new_state);
    }

    pub fn set_state(&self, state: S) {
        self.state_holder.write().unwrap().set_state(state);
    }
}
