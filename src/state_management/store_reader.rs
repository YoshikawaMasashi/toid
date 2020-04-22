use std::sync::Arc;

use super::super::resource_management::resource_manager::ResourceManager;
use super::store::Store;

pub trait StoreReader<O, E, S, SE> {
    fn new() -> Self
    where
        Self: Sized;
    fn read(&mut self, store: Arc<Store<S, SE>>, resource_manager: Arc<ResourceManager>) -> O;
    fn apply(&mut self, event: E);
}
