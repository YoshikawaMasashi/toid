use std::sync::Arc;

use super::super::super::resource_management::resource_manager::ResourceManager;
use super::super::super::state_management::serialize::Serialize;
use super::super::super::state_management::state::State;
use super::super::super::state_management::store::Store;
use super::super::player::Player;

pub struct LocalPlayer<S, E> {
    store: Arc<Store<S, E>>,
    resource_manager: Arc<ResourceManager>,
}

impl<S: State<E>, E: Sized + Serialize<E>> LocalPlayer<S, E> {
    pub fn new(store: Arc<Store<S, E>>, resource_manager: Arc<ResourceManager>) -> Self {
        Self {
            store: store,
            resource_manager: resource_manager,
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
