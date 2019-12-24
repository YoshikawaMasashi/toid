use std::boxed::Box;
use std::sync::Arc;
use std::sync::RwLock;

use super::super::state_management::reducer::Reduce;
use super::super::state_management::reducer::Reducer;
use super::super::state_management::store::Store;

pub struct DefaultReducer<T, S> {
    store: Arc<RwLock<Box<dyn Store<T>>>>,
    reduce: Box<dyn Reduce<T, S>>,
}

impl<T: Clone, S> DefaultReducer<T, S> {
    pub fn new(store: Arc<RwLock<Box<dyn Store<T>>>>, reduce: Box<dyn Reduce<T, S>>) -> Self {
        DefaultReducer { store, reduce }
    }
}

impl<T: Clone, S> Reducer<T, S> for DefaultReducer<T, S> {
    fn reduce(&self, event: S) {
        let mut store = self.store.write().unwrap();
        let state = store.get_state();
        let new_state = self.reduce.reduce(state, event);
        store.update_state(new_state);
    }
}
