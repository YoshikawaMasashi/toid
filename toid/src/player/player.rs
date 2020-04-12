use std::marker::PhantomData;

use super::super::resource_management::resource_manager::ResourceManager;
use super::super::state_management::store::Store;

pub struct Player<S, E> {
    pub store: Box<dyn Store<S, E>>,
    pub resource_manager: ResourceManager,
    state_marker: PhantomData<S>,
    event_marker: PhantomData<E>,
}

impl<S, E> Player<S, E> {
    pub fn new(store: Box<dyn Store<S, E>>, resource_manager: ResourceManager) -> Self {
        Self {
            store,
            resource_manager,
            state_marker: PhantomData,
            event_marker: PhantomData,
        }
    }

    pub fn send_event(&self, event: E) {
        self.store.update_state(event);
    }
}
