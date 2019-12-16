extern crate im;

use im::hashmap::HashMap;
use std::boxed::Box;
use std::sync::Arc;
use std::sync::RwLock;

trait Event {}

enum StateType {
    HashMapState(HashMapState),
    i32(i32),
    f32(f32),
}

impl Clone for StateType {
    fn clone(&self) -> StateType {
        match self {
            StateType::HashMapState(m) => StateType::HashMapState(HashMapState {
                state_map: m.state_map.clone(),
            }),
            StateType::i32(i) => StateType::i32(*i),
            StateType::f32(f) => StateType::f32(*f),
        }
    }
}

struct HashMapState {
    state_map: HashMap<String, StateType>,
}

impl HashMapState {
    fn get_by_address(&self, address: String) -> StateType {
        StateType::i32(0)
    }

    fn update(&self, event: Box<dyn Event>) -> StateType {
        let new_state_map = self.state_map.update(String::from("a"), StateType::i32(0));
        let new_state = HashMapState {
            state_map: new_state_map,
        };
        StateType::HashMapState(new_state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_works() {}
}
