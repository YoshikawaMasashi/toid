use std::marker::PhantomData;
use std::sync::Arc;

use super::super::resource_management::resource_manager::ResourceManager;
use super::super::state_management::serialize::Serialize;
use super::super::state_management::state::State;
use super::super::state_management::store::Store;

pub struct Player<S, E> {
    pub store: Arc<Store<S, E>>,
    pub resource_manager: Arc<ResourceManager>,
    state_marker: PhantomData<S>,
    event_marker: PhantomData<E>,
}

impl<S: State<E>, E: Sized + Serialize<E>> Player<S, E> {
    pub fn new(store: Arc<Store<S, E>>, resource_manager: Arc<ResourceManager>) -> Self {
        Self {
            store: store,
            resource_manager: resource_manager,
            state_marker: PhantomData,
            event_marker: PhantomData,
        }
    }

    pub fn send_event(&self, event: E) {
        self.store.update_state(event);
    }
}
