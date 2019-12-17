use im::hashmap::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use super::state_management::hashmap_state::HashMapState;
use super::state_management::reducer::Reduce;
use super::state_management::reducer::Reducer;
use super::state_management::state::State;
use super::state_management::store::Store;

struct Event {
    key: String,
    value: i32,
}

struct HashMapReduce {}

impl Reduce<Event> for HashMapReduce {
    fn reduce(&self, state: State, event: Event) -> State {
        state
            .unwrap_manual_state()
            .update(event.key, State::i32(event.value))
            .unwrap()
    }
}

#[test]
fn state_works() {
    let initial_state = State::ManualState(Arc::new(HashMapState::new()));
    let store = Arc::new(RwLock::new(Store::new(initial_state)));
    assert_eq!(
        store
            .read()
            .unwrap()
            .get_state()
            .unwrap_manual_state()
            .contains_address(String::from("a")),
        false
    );

    let reduce = Box::new(HashMapReduce {});
    let reducer = Reducer::new(Arc::clone(&store), reduce);
    reducer.reduce(Event {
        key: String::from("a"),
        value: 1,
    });
    assert_eq!(
        store
            .read()
            .unwrap()
            .get_state()
            .unwrap_manual_state()
            .contains_address(String::from("a")),
        true
    );
    assert_eq!(
        store
            .read()
            .unwrap()
            .get_state()
            .unwrap_manual_state()
            .get_by_address(String::from("a"))
            .unwrap()
            .unwrap_i32(),
        1
    );
}
