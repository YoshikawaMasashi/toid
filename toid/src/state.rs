use im::hashmap::HashMap;
use std::boxed::Box;
use std::cell::RefCell;
use std::option::Option;
use std::rc::Rc;

struct Store<T> {
    state: Rc<T>,
}

impl<T> Store<T> {
    fn new(initial_state: T) -> Self {
        let state = Rc::new(initial_state);
        Store { state }
    }

    fn update_state(&mut self, state: T) {
        let new_state = Rc::new(state);
        self.state = new_state;
    }

    fn get_state(&self) -> Rc<T> {
        return Rc::clone(&self.state);
    }
}

trait Reduce<T, S> {
    fn reduce(&self, state: Rc<T>, event: &S) -> (T);
}

struct Reducer<T, S> {
    store: Rc<RefCell<Store<T>>>,
    reduce: Box<Reduce<T, S>>,
}

impl<T, S> Reducer<T, S> {
    fn new(store: Rc<RefCell<Store<T>>>, reduce: Box<Reduce<T, S>>) -> Self {
        Reducer { store, reduce }
    }

    fn reduce(&self, event: &S) {
        let mut store = self.store.borrow_mut();
        let state = store.get_state();
        let new_state = self.reduce.reduce(state, event);
        store.update_state(new_state);
    }
}

struct Event {
    key: i32,
    value: i32,
}

struct HashMapReduce {}

impl Reduce<HashMap<i32, i32>, Event> for HashMapReduce {
    fn reduce(&self, state: Rc<HashMap<i32, i32>>, event: &Event) -> (HashMap<i32, i32>) {
        state.update(event.key, event.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_works() {
        let initial_state: HashMap<i32, i32> = HashMap::new();
        let store = Rc::new(RefCell::new(Store::new(initial_state)));
        assert_eq!(store.borrow().get_state().len(), 0);

        let reduce = Box::new(HashMapReduce {});
        let reducer = Reducer::new(Rc::clone(&store), reduce);

        reducer.reduce(&Event { key: 0, value: 1 });
        assert_eq!(store.borrow().get_state().len(), 1);
        assert_eq!(*store.borrow().get_state().get(&(0 as i32)).unwrap(), 1);

        reducer.reduce(&Event { key: 1, value: 345 });
        assert_eq!(store.borrow().get_state().len(), 2);
        assert_eq!(*store.borrow().get_state().get(&(1 as i32)).unwrap(), 345);

        reducer.reduce(&Event { key: 1, value: 2 });
        assert_eq!(store.borrow().get_state().len(), 2);
        assert_eq!(*store.borrow().get_state().get(&(1 as i32)).unwrap(), 2);
    }
}
