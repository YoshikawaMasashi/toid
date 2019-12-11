use super::state::*;
use im::hashmap::HashMap;
use std::boxed::Box;
use std::cell::RefCell;
use std::rc::Rc;

struct Event {
    key: i32,
    value: i32,
}

struct HashMapReduce {}

impl Reduce<HashMap<i32, i32>, Event> for HashMapReduce {
    fn reduce(&self, state: Rc<HashMap<i32, i32>>, event: &Event) -> HashMap<i32, i32> {
        state.update(event.key, event.value)
    }
}

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
