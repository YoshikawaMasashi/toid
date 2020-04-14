use std::sync::Arc;
use std::sync::RwLock;

use super::super::resource_management::resource_manager::ResourceManager;
use super::super::state_management::store::Store;
use super::super::state_management::store_reader::StoreReader;

pub trait Player<S, E, R: StoreReader<O, RE, S, E>, O, RE> {
    fn get_store(&self) -> Arc<Store<S, E>>;
    fn get_resource_manager(&self) -> Arc<ResourceManager>;
    fn get_reader(&self) -> Arc<RwLock<R>>;
    fn send_event(&self, event: E);
}
