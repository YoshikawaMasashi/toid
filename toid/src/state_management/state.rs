extern crate im;

use im::hashmap::HashMap;
use std::boxed::Box;
use std::sync::Arc;
use std::sync::RwLock;

enum State {
    ManualState(Arc<dyn ManualState>),
    i32(i32),
    f32(f32),
}

impl Clone for State {
    fn clone(&self) -> Self {
        match self {
            State::ManualState(m) => State::ManualState(Arc::clone(m)),
            State::i32(i) => State::i32(*i),
            State::f32(f) => State::f32(*f),
        }
    }
}

trait ManualState {
    fn get_by_address(&self, address: String) -> Result<State, String>;
    fn update(&self, address: String, value: State) -> Result<State, String>;
}

struct HashMapState {
    state_map: HashMap<String, State>,
}

fn split_by_first_slash(address: String) -> Result<(String, String), String> {
    match address.find('/') {
        Some(i) => Ok((
            address[0..i].to_string(),
            address[i + 1..address.len()].to_string(),
        )),
        None => Err(address),
    }
}

impl ManualState for HashMapState {
    fn get_by_address(&self, address: String) -> Result<State, String> {
        match split_by_first_slash(address) {
            Ok((first, other)) => match self.state_map.get(&first) {
                Some(s) => match s {
                    State::ManualState(m) => m.get_by_address(other),
                    _ => Err(String::from("dame")),
                },
                None => Err(String::from("not exist")),
            },
            Err(first) => match self.state_map.get(&first) {
                Some(s) => Ok(s.clone()),
                None => Err(String::from("not exist")),
            },
        }
    }

    fn update(&self, address: String, value: State) -> Result<State, String> {
        match split_by_first_slash(address) {
            Ok((first, other)) => match self.state_map.get(&first) {
                Some(s) => match s {
                    State::ManualState(m) => match m.update(other, value) {
                        Ok(new_m) => {
                            let new_state_map = self.state_map.update(first, new_m);
                            let new_state = HashMapState {
                                state_map: new_state_map,
                            };
                            Ok(State::ManualState(Arc::new(new_state)))
                        }
                        _ => Err(String::from("dame")),
                    },
                    _ => Err(String::from("dame")),
                },
                None => Err(String::from("not exist")),
            },
            Err(first) => {
                let new_state_map = self.state_map.update(first, value);
                let new_state = HashMapState {
                    state_map: new_state_map,
                };
                Ok(State::ManualState(Arc::new(new_state)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_works() {}
}

/*
enum StateType {
    HashMapState(Arc<HashMapState>),
    i32(i32),
    f32(f32),
}

impl Clone for StateType {
    fn clone(&self) -> Self {
        match self {
            StateType::HashMapState(m) => StateType::HashMapState(Arc::clone(m)),
            StateType::i32(i) => StateType::i32(*i),
            StateType::f32(f) => StateType::f32(*f),
        }
    }
}

struct HashMapState {
    state_map: HashMap<String, StateType>,
}

impl Clone for HashMapState {
    fn clone(&self) -> Self {
        HashMapState {
            state_map: self.state_map.clone(),
        }
    }
}

impl HashMapState {
    fn get_by_address(&self, address: String) -> Result<StateType, String> {
        let splited_address: Vec<&str> = address.split(',').collect();
        let now_state = StateType::HashMapState(Arc::new(self.clone()));

        for name in splited_address {
            match now_state.clone() {
                StateType::HashMapState(m) => m.state_map.get(name).unwrap(),
                StateType::i32(i) => &StateType::i32(i),
                StateType::f32(f) => &StateType::f32(f),
            };
        }

        Ok(StateType::i32(0))
    }

    fn update(&self, address: String, value: StateType) -> StateType {
        let new_state_map = self.state_map.update(String::from("a"), StateType::i32(0));
        let new_state = HashMapState {
            state_map: new_state_map,
        };
        StateType::HashMapState(Arc::new(new_state))
    }
}
*/
