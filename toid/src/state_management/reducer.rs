use std::boxed::Box;
use std::sync::Arc;
use std::sync::RwLock;

use super::state::State;
use super::store::Store;

/// Reduceでは、あるeventが会った時に、stateの変更の仕方をユーザーが決めることができるインターフェースです。
pub trait Reduce<T> {
    fn reduce(&self, state: State, event: T) -> State;
}

/// Reducerでは、ユーザーが実装したReduceによって、Storeに変更するように要請します。
/// Storeではstateを変更できるメソッドは非公開であるので、Reducerでしか変更できません。
pub struct Reducer<T> {
    store: Arc<RwLock<Store>>,
    reduce: Box<dyn Reduce<T>>,
}

impl<T> Reducer<T> {
    pub fn new(store: Arc<RwLock<Store>>, reduce: Box<dyn Reduce<T>>) -> Self {
        Reducer { store, reduce }
    }

    pub fn reduce(&self, event: T) {
        let mut store = self.store.write().unwrap();
        let state = store.get_state();
        let new_state = self.reduce.reduce(state, event);
        store.update_state(new_state);
    }
}
