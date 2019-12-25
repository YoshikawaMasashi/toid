use std::boxed::Box;
use std::sync::Arc;
use std::sync::RwLock;
use ws::connect;
use ws::listen;
use ws::{Builder, CloseCode, Handler, Handshake, Message, Result, Sender, Settings};

use super::super::state_management::reducer::Reduce;
use super::super::state_management::reducer::Reducer;
use super::super::state_management::serialize::Serialize;
use super::super::state_management::store::Store;

// TODO: reduceがユーザーから来た時に、メッセージをサーバーに飛ばすやつ
// TODO: メッセージがサーバーから来た時に、storeを変更するやつ

pub struct WebSocketReducerClient<T, S: Serialize> {
    methods: Arc<RwLock<WebSocketReducerMethods<T, S>>>,
}

impl<T, S: Serialize> Handler for WebSocketReducerClient<T, S> {
    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        self.methods.write().unwrap().on_open(handshake)
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        self.methods.write().unwrap().on_message(msg)
    }
}

pub struct WebSocketReducerMethods<T, S: Serialize> {
    store: Arc<RwLock<Box<dyn Store<T>>>>,
    reduce: Box<dyn Reduce<T, S>>,
    out: Sender,
}

impl<T, S: Serialize> WebSocketReducerMethods<T, S> {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        Ok(())
    }

    fn send(&self, msg: &str) {
        self.out.send(msg).unwrap();
    }
}

pub struct WebSocketReducer<T, S: Serialize> {
    methods: Arc<RwLock<WebSocketReducerMethods<T, S>>>,
}

impl<T: Clone, S: Serialize> Reducer<T, S> for WebSocketReducer<T, S> {
    fn reduce(&self, event: S) {

    }
}

impl<T: Clone, S: Serialize> WebSocketReducer<T, S> {
    pub fn connect(&self) {
        connect("ws://127.0.0.1:3012", |out| WebSocketReducerClient {
            methods: Arc::clone(&self.methods),
        })
        .unwrap();
    }
}

/*
pub struct WebSocketReducer<T, S: Serialize> {
    store: Arc<RwLock<Box<dyn Store<T>>>>,
    reduce: Box<dyn Reduce<T, S>>,
    out: Option<Arc<Sender>>,
}

impl<T: Clone, S: Serialize> Handler for WebSocketReducer<T, S> {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.out.as_ref().unwrap().send("Hello WebSocket")
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Got message: {}", msg);
        self.out.as_ref().unwrap().close(CloseCode::Normal)
    }
}

impl<T: Clone, S: Serialize> Reducer<T, S> for WebSocketReducer<T, S> {
    fn reduce(&self, event: S) {
        let mut store = self.store.write().unwrap();
        let state = store.get_state();
        let new_state = self.reduce.reduce(state, event);
        store.update_state(new_state);
    }
}

impl<T: Clone, S: Serialize> WebSocketReducer<T, S> {
    pub fn new(store: Arc<RwLock<Box<dyn Store<T>>>>, reduce: Box<dyn Reduce<T, S>>) -> Self {
        WebSocketReducer {
            store,
            reduce,
            out: None,
        }
    }

    pub fn connect(&mut self) {
        connect("ws://127.0.0.1:3012", move |out| {
            let not_option_out = Arc::new(out);
            self.out = Some(not_option_out);
            self
        })
        .unwrap();
    }
}
*/

pub struct WebSocketReducerServer {}

impl WebSocketReducerServer {
    pub fn new() -> Self {
        WebSocketReducerServer {}
    }

    pub fn run() {
        listen("127.0.0.1:3012", |out| move |msg| out.broadcast(msg)).unwrap();
    }
}
