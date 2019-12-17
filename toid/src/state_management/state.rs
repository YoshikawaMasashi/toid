use std::sync::Arc;

pub enum State {
    ManualState(Arc<dyn ManualState>),
    I32(i32),
    F32(f32),
    Bool(bool),
    Usize(usize),
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
            State::I32(i) => *i,
            _ => panic!("is not i32"),
        }
    }

    pub fn unwrap_f32(&self) -> f32 {
        match self {
            State::F32(f) => *f,
            _ => panic!("is not f32"),
        }
    }

    pub fn unwrap_bool(&self) -> bool {
        match self {
            State::Bool(b) => *b,
            _ => panic!("is not bool"),
        }
    }

    pub fn unwrap_usize(&self) -> usize {
        match self {
            State::Usize(u) => *u,
            _ => panic!("is not usize"),
        }
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        match self {
            State::ManualState(m) => State::ManualState(Arc::clone(m)),
            State::I32(i) => State::I32(*i),
            State::F32(f) => State::F32(*f),
            State::Bool(b) => State::Bool(*b),
            State::Usize(u) => State::Usize(*u),
        }
    }
}

pub trait ManualState {
    fn get_by_address(&self, address: String) -> Result<State, String>;
    fn update(&self, address: String, value: State) -> Result<State, String>;
    fn contains_address(&self, address: String) -> bool;
}
