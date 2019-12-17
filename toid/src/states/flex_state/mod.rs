pub mod hashmap_state;

use std::sync::Arc;

pub enum FlexState {
    ManualState(Arc<dyn ManualState>),
    I32(i32),
    F32(f32),
    Bool(bool),
    Usize(usize),
}

impl FlexState {
    pub fn unwrap_manual_state(&self) -> Arc<dyn ManualState> {
        match self {
            FlexState::ManualState(m) => Arc::clone(m),
            _ => panic!("is not ManualState"),
        }
    }

    pub fn unwrap_i32(&self) -> i32 {
        match self {
            FlexState::I32(i) => *i,
            _ => panic!("is not i32"),
        }
    }

    pub fn unwrap_f32(&self) -> f32 {
        match self {
            FlexState::F32(f) => *f,
            _ => panic!("is not f32"),
        }
    }

    pub fn unwrap_bool(&self) -> bool {
        match self {
            FlexState::Bool(b) => *b,
            _ => panic!("is not bool"),
        }
    }

    pub fn unwrap_usize(&self) -> usize {
        match self {
            FlexState::Usize(u) => *u,
            _ => panic!("is not usize"),
        }
    }
}

impl Clone for FlexState {
    fn clone(&self) -> Self {
        match self {
            FlexState::ManualState(m) => FlexState::ManualState(Arc::clone(m)),
            FlexState::I32(i) => FlexState::I32(*i),
            FlexState::F32(f) => FlexState::F32(*f),
            FlexState::Bool(b) => FlexState::Bool(*b),
            FlexState::Usize(u) => FlexState::Usize(*u),
        }
    }
}

pub trait ManualState {
    fn get_by_address(&self, address: String) -> Result<FlexState, String>;
    fn update(&self, address: String, value: FlexState) -> Result<FlexState, String>;
    fn contains_address(&self, address: String) -> bool;
}
