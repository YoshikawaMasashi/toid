use std::sync::Arc;

pub enum State {
    ManualState(Arc<dyn ManualState>),
    i32(i32),
    f32(f32),
}

impl State {
    pub fn unwrap_manual_state(&self) -> Arc<dyn ManualState> {
        match self {
            State::ManualState(m) => Arc::clone(m),
            _ => panic!("is not ManualState"),
        }
    }

    pub fn unwrap_i32(&self) -> i32 {
        match self {
            State::i32(i) => *i,
            _ => panic!("is not i32"),
        }
    }

    pub fn unwrap_f32(&self) -> f32 {
        match self {
            State::f32(f) => *f,
            _ => panic!("is not f32"),
        }
    }
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

pub trait ManualState {
    fn get_by_address(&self, address: String) -> Result<State, String>;
    fn update(&self, address: String, value: State) -> Result<State, String>;
    fn contains_address(&self, address: String) -> bool;
}
