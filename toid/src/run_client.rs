use std::boxed::Box;
use std::sync::Arc;
use std::sync::RwLock;

use serde::{Deserialize, Serialize};

use toid::reducers::websocket_reducer::WebSocketReducer;
use toid::state_management::reducer::Reduce;
use toid::state_management::reducer::Reducer;
use toid::state_management::serialize;
use toid::state_management::store::Store;
use toid::stores::default_store::DefaultStore;

struct NumState {
    num: i32,
}

impl Clone for NumState {
    fn clone(&self) -> Self {
        NumState { num: self.num }
    }
}

impl NumState {
    fn new() -> Self {
        NumState { num: 0 }
    }
}
#[derive(Serialize, Deserialize)]
enum NumStateEvent {
    Increment,
}

impl serialize::Serialize for NumStateEvent {
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    fn deserialize(serialized: String) -> Self {
        serde_json::from_str(serialized.as_str()).unwrap()
    }
}

struct NumStateReduce {}

impl NumStateReduce {
    fn new() -> Self {
        NumStateReduce {}
    }
}
impl Reduce<NumState, NumStateEvent> for NumStateReduce {
    fn reduce(&self, state: NumState, event: NumStateEvent) -> NumState {
        match event {
            NumStateEvent::Increment => NumState { num: state.num + 1 },
        }
    }
}

fn main() {
    let store: Box<dyn Store<NumState>> = Box::new(DefaultStore::new(NumState::new()));
    let store = Arc::new(RwLock::new(store));

    let reduce = Box::new(NumStateReduce::new());
    let reducer = WebSocketReducer::new(Arc::clone(&store), reduce);

    loop {
        println!("input the command: i: increment q: quit g: get number");
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let s = s.trim_right().to_owned();
        match s.as_str() {
            "i" => reducer.reduce(NumStateEvent::Increment),
            "q" => break,
            "g" => println!("{}", store.read().unwrap().get_state().num),
            _ => {}
        }
    }
}
