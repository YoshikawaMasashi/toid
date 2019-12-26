use std::boxed::Box;
use std::marker::{Send, Sync};
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

use ws::connect;
use ws::listen;
use ws::{Handler, Handshake, Message, Result, Sender};

use super::super::state_management::reducer::Reduce;
use super::super::state_management::reducer::Reducer;
use super::super::state_management::serialize::Serialize;
use super::super::state_management::store::Store;

pub struct WebSocketReducerClient<T, S> {
    methods: Arc<RwLock<WebSocketReducerMethods<T, S>>>,
}

impl<T: Clone, S: Serialize> Handler for WebSocketReducerClient<T, S> {
    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        self.methods.write().unwrap().on_open(handshake)
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        self.methods.write().unwrap().on_message(msg)
    }
}

pub struct WebSocketReducerMethods<T, S> {
    store: Arc<RwLock<Box<dyn Store<T> + Sync + Send>>>,
    reduce: Box<dyn Reduce<T, S> + Sync + Send>,
    out: Option<Sender>,
}

impl<T: Clone, S: Serialize> WebSocketReducerMethods<T, S> {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        self.reduce(S::deserialize(msg.to_string()));
        Ok(())
    }

    fn send(&self, event: S) {
        if let Some(out) = self.out.as_ref() {
            out.send(event.serialize()).unwrap();
        }
    }

    fn reduce(&self, event: S) {
        let mut store = self.store.write().unwrap();
        let state = store.get_state();
        let new_state = self.reduce.reduce(state, event);
        store.update_state(new_state);
    }

    fn set_sender(&mut self, out: Sender) {
        self.out = Some(out);
    }
}

pub struct WebSocketReducer<T, S> {
    methods: Arc<RwLock<WebSocketReducerMethods<T, S>>>,
}

impl<T: Clone, S: Serialize> Reducer<T, S> for WebSocketReducer<T, S> {
    fn reduce(&self, event: S) {
        self.methods.read().unwrap().send(event);
    }
}

impl<T: 'static + Clone, S: 'static + Serialize> WebSocketReducer<T, S> {
    pub fn new(
        store: Arc<RwLock<Box<dyn Store<T> + Sync + Send>>>,
        reduce: Box<dyn Reduce<T, S> + Sync + Send>,
    ) -> Self {
        let methods = WebSocketReducerMethods {
            store: store,
            reduce: reduce,
            out: None,
        };
        let methods = Arc::new(RwLock::new(methods));
        WebSocketReducer { methods }
    }

    pub fn connect(&mut self) {
        let methods = Arc::clone(&self.methods);
        thread::spawn(move || {
            connect("ws://127.0.0.1:3012", move |out| {
                methods.write().unwrap().set_sender(out);
                WebSocketReducerClient {
                    methods: Arc::clone(&methods),
                }
            })
            .unwrap();
        });
    }
}

pub struct WebSocketReducerServer {}

impl WebSocketReducerServer {
    pub fn new() -> Self {
        WebSocketReducerServer {}
    }

    pub fn run(&self) {
        listen("127.0.0.1:3012", |out| move |msg| out.broadcast(msg)).unwrap();
    }
}
