use std::boxed::Box;
use std::sync::Arc;
use std::sync::RwLock;

use super::store::Store;

/// Reduceでは、あるeventが会った時に、stateの変更の仕方をユーザーが決めることができるインターフェースです。
pub trait Reduce<T, S> {
    fn reduce(&self, state: Arc<T>, event: &S) -> T;
}

/// Reducerでは、ユーザーが実装したReduceによって、Storeに変更するように要請します。
/// Storeではstateを変更できるメソッドは非公開であるので、Reducerでしか変更できません。
pub struct Reducer<T, S> {
    store: Arc<RwLock<Store<T>>>,
    reduce: Box<dyn Reduce<T, S>>,
}

impl<T, S> Reducer<T, S> {
    pub fn new(store: Arc<RwLock<Store<T>>>, reduce: Box<dyn Reduce<T, S>>) -> Self {
        Reducer { store, reduce }
    }

    pub fn reduce(&self, event: &S) {
        let mut store = self.store.write().unwrap();
        let state = store.get_state();
        let new_state = self.reduce.reduce(state, event);
        store.update_state(new_state);
    }
}
