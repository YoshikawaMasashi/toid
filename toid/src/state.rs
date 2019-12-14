//! stateモジュールでは、イベント駆動の仕組みを実装しています。
//! Observerパターンや、Webのフロントエンドで使われるFluxと基本的には似たような仕組みで動いています。

use std::boxed::Box;
use std::sync::Arc;
use std::sync::RwLock;

/// Storeはstateを保持し、必要に応じてアップデートをする窓口を提供します。
pub struct Store<T> {
    state: Arc<T>,
}

impl<T> Store<T> {
    pub fn new(initial_state: T) -> Self {
        let state = Arc::new(initial_state);
        Store { state }
    }

    fn update_state(&mut self, state: T) {
        let new_state = Arc::new(state);
        self.state = new_state;
    }

    pub fn get_state(&self) -> Arc<T> {
        return Arc::clone(&self.state);
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use im::hashmap::HashMap;

    struct Event {
        key: i32,
        value: i32,
    }

    struct HashMapReduce {}

    impl Reduce<HashMap<i32, i32>, Event> for HashMapReduce {
        fn reduce(&self, state: Arc<HashMap<i32, i32>>, event: &Event) -> HashMap<i32, i32> {
            state.update(event.key, event.value)
        }
    }

    #[test]
    fn state_works() {
        let initial_state: HashMap<i32, i32> = HashMap::new();
        let store = Arc::new(RwLock::new(Store::new(initial_state)));
        assert_eq!(store.read().unwrap().get_state().len(), 0);

        let reduce = Box::new(HashMapReduce {});
        let reducer = Reducer::new(Arc::clone(&store), reduce);

        reducer.reduce(&Event { key: 0, value: 1 });
        assert_eq!(store.read().unwrap().get_state().len(), 1);
        assert_eq!(
            *store.read().unwrap().get_state().get(&(0 as i32)).unwrap(),
            1
        );

        reducer.reduce(&Event { key: 1, value: 345 });
        assert_eq!(store.read().unwrap().get_state().len(), 2);
        assert_eq!(
            *store.read().unwrap().get_state().get(&(1 as i32)).unwrap(),
            345
        );

        reducer.reduce(&Event { key: 1, value: 2 });
        assert_eq!(store.read().unwrap().get_state().len(), 2);
        assert_eq!(
            *store.read().unwrap().get_state().get(&(1 as i32)).unwrap(),
            2
        );
    }
}
