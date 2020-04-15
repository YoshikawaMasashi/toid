use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::RwLock;

use super::super::super::resource_management::resource_manager::{
    ResourceManager, ResourceManagerEvent,
};
use super::super::super::state_management::serialize::Serialize;
use super::super::super::state_management::state::State;
use super::super::super::state_management::store::Store;
use super::super::super::state_management::store_reader::StoreReader;
use super::super::player::Player;

pub struct LocalPlayer<S, E, R, O, RE> {
    store: Arc<Store<S, E>>,
    resource_manager: Arc<ResourceManager>,
    reader: Arc<RwLock<R>>,
    output_marker: PhantomData<O>,
    reader_event_marker: PhantomData<RE>,
}

impl<S: State<E>, E: Sized + Serialize<E>, R: StoreReader<O, RE, S, E>, O, RE: Sized>
    LocalPlayer<S, E, R, O, RE>
{
    pub fn new() -> Self {
        Self {
            store: Arc::new(Store::new(S::new())),
            resource_manager: Arc::new(ResourceManager::new()),
            reader: Arc::new(RwLock::new(R::new())),
            output_marker: PhantomData,
            reader_event_marker: PhantomData,
        }
    }
}

impl<S: State<E>, E: Sized + Serialize<E>, R: StoreReader<O, RE, S, E>, O, RE>
    Player<S, E, R, O, RE> for LocalPlayer<S, E, R, O, RE>
{
    fn get_store(&self) -> Arc<Store<S, E>> {
        Arc::clone(&self.store)
    }

    fn get_resource_manager(&self) -> Arc<ResourceManager> {
        Arc::clone(&self.resource_manager)
    }

    fn get_reader(&self) -> Arc<RwLock<R>> {
        Arc::clone(&self.reader)
    }

    fn send_event(&self, event: E) {
        self.store.update_state(event);
    }

    fn send_reader_event(&self, event: RE) {
        self.reader.write().unwrap().apply(event);
    }

    fn send_resource_event(&self, event: ResourceManagerEvent) {
        if let Err(error) = self.resource_manager.apply(event) {
            println!("send resource event error !: {}", error);
        }
    }
}
