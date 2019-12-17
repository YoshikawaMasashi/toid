use std::sync::Arc;

use im::hashmap::HashMap;

use super::state::ManualState;
use super::state::State;

fn split_by_first_slash(address: String) -> Result<(String, String), String> {
    match address.find('/') {
        Some(i) => Ok((
            address[0..i].to_string(),
            address[i + 1..address.len()].to_string(),
        )),
        None => Err(address),
    }
}

pub struct HashMapState {
    state_map: HashMap<String, State>,
}

impl HashMapState {
    pub fn new() -> Self {
        HashMapState {
            state_map: HashMap::new(),
        }
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

    fn contains_address(&self, address: String) -> bool {
        match split_by_first_slash(address) {
            Ok((first, other)) => match self.state_map.get(&first) {
                Some(s) => match s {
                    State::ManualState(m) => m.contains_address(other),
                    _ => false,
                },
                None => false,
            },
            Err(first) => self.state_map.contains_key(&first),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_works() {
        let root_state = HashMapState::new();
        let root_state = root_state
            .update(
                String::from("music"),
                State::ManualState(Arc::new(HashMapState::new())),
            )
            .unwrap()
            .unwrap_manual_state();

        assert_eq!(
            root_state.contains_address(String::from("music/pitch")),
            false
        );
        let root_state = root_state
            .update(String::from("music/pitch"), State::i32(60))
            .unwrap()
            .unwrap_manual_state();

        assert_eq!(
            root_state.contains_address(String::from("music/pitch")),
            true
        );
        assert_eq!(
            root_state
                .get_by_address(String::from("music/pitch"))
                .unwrap()
                .unwrap_i32(),
            60
        );
    }
}
