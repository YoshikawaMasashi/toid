use std::marker::PhantomData;
use std::sync::Arc;

use super::super::resource_management::resource_manager::ResourceManager;
use super::super::state_management::serialize::Serialize;
use super::super::state_management::state::State;
use super::super::state_management::store::Store;
use super::player::Player;

pub struct LocalPlayer<S, E> {
    store: Arc<Store<S, E>>,
    resource_manager: Arc<ResourceManager>,
    state_marker: PhantomData<S>,
    event_marker: PhantomData<E>,
}

impl<S: State<E>, E: Sized + Serialize<E>> LocalPlayer<S, E> {
    pub fn new(store: Arc<Store<S, E>>, resource_manager: Arc<ResourceManager>) -> Self {
        Self {
            store: store,
            resource_manager: resource_manager,
            state_marker: PhantomData,
            event_marker: PhantomData,
        }
    }
}

impl<S: State<E>, E: Sized + Serialize<E>> Player<S, E> for LocalPlayer<S, E> {
    fn get_store(&self) -> Arc<Store<S, E>> {
        Arc::clone(&self.store)
    }

    fn get_resource_manager(&self) -> Arc<ResourceManager> {
        Arc::clone(&self.resource_manager)
    }

    fn send_event(&self, event: E) {
        self.store.update_state(event);
    }
}
