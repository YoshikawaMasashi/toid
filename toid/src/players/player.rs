use std::sync::Arc;

use super::super::resource_management::resource_manager::ResourceManager;
use super::super::state_management::store::Store;

pub trait Player<S, E> {
    fn get_store(&self) -> Arc<Store<S, E>>;
    fn get_resource_manager(&self) -> Arc<ResourceManager>;
    fn send_event(&self, event: E);
}
