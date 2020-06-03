use std::fs::File;
use std::io::{Read, Write};
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

impl<S: State<E> + Serialize<S>, E: Sized + Serialize<E>, R: StoreReader<O, RE, S, E>, O, RE>
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

    fn send_event(&self, event: E) -> Result<(), String> {
        self.store.update_state(event)?;
        Ok(())
    }

    fn send_reader_event(&self, event: RE) -> Result<(), String> {
        self.reader
            .write()
            .map_err(|_| "rwlock error")?
            .apply(event);
        Ok(())
    }

    fn send_resource_event(&self, event: ResourceManagerEvent) -> Result<(), String> {
        self.resource_manager.apply(event)?;
        Ok(())
    }

    fn save_state(&self, path: String) -> Result<(), String> {
        let serialized_state: String = self.store.get_state()?.serialize()?;
        let mut file = File::create(path).or_else(|e| Err(e.to_string()))?;
        file.write_all(serialized_state.as_bytes())
            .or_else(|e| Err(e.to_string()))?;
        Ok(())
    }

    fn load_state(&self, path: String) -> Result<(), String> {
        let mut file = File::open(path).or_else(|e| Err(e.to_string()))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .or_else(|e| Err(e.to_string()))?;
        let state: S = S::deserialize(contents)?;
        self.store.set_state(state)?;
        Ok(())
    }
}
