use std::marker::PhantomData;
use std::marker::{Send, Sync};
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

use ws;

use super::super::super::state_management::reducer::Reducer;
use super::super::super::state_management::serialize::Serialize;
use super::super::super::state_management::state_holder::StateHolder;
use super::super::super::state_management::store::Store;

pub struct SenderHolder {
    out: Option<ws::Sender>,
}

pub struct WebSocketStore<S, E, R> {
    sender_holder: Arc<RwLock<SenderHolder>>,
    state_holder: Arc<RwLock<StateHolder<S>>>,
    event_marker: PhantomData<E>,
    reducer: Arc<R>,
}

pub struct WebSocketStoreHandler<S, E, R> {
    state_holder: Arc<RwLock<StateHolder<S>>>,
    reducer: Arc<R>,
    event_marker: PhantomData<E>,
}

impl SenderHolder {
    fn new() -> Self {
        Self { out: None }
    }

    fn set_sender(&mut self, out: ws::Sender) {
        self.out = Some(out);
    }
}

impl<
        S: 'static + Send + Sync,
        E: Sized + Serialize<E> + Send + Sync,
        R: Reducer<S, E> + 'static + Send + Sync,
    > WebSocketStore<S, E, R>
{
    pub fn new(state: S, reducer: R) -> Self {
        Self {
            sender_holder: Arc::new(RwLock::new(SenderHolder::new())),
            state_holder: Arc::new(RwLock::new(StateHolder::new(state))),
            event_marker: PhantomData,
            reducer: Arc::new(reducer),
        }
    }

    pub fn connect(&mut self, connect_address: String) {
        let state_holder = Arc::clone(&self.state_holder);
        let reducer = Arc::clone(&self.reducer);
        let sender_holder = Arc::clone(&self.sender_holder);
        thread::spawn(move || {
            if let Err(error) = ws::connect(connect_address, |out| {
                sender_holder.write().unwrap().set_sender(out);
                let handler: WebSocketStoreHandler<S, E, R> = WebSocketStoreHandler {
                    state_holder: Arc::clone(&state_holder),
                    reducer: Arc::clone(&reducer),
                    event_marker: PhantomData,
                };
                handler
            }) {
                println!("Failed to create WebSocket due to: {:?}", error);
            }
        });
    }
}

impl<S, E: Sized + Serialize<E>, R> Store<S, E> for WebSocketStore<S, E, R> {
    fn get_state(&self) -> Arc<S> {
        Arc::clone(&self.state_holder.read().unwrap().get_state())
    }

    fn update_state(&self, event: E) {
        if let Some(out) = &self.sender_holder.read().unwrap().out {
            let serialized = event.serialize().unwrap();
            out.send(serialized).unwrap();
        } else {
            println!("out is None")
        }
    }
}

impl<S, E: Sized + Serialize<E>, R: Reducer<S, E>> ws::Handler for WebSocketStoreHandler<S, E, R> {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        println!("connected");
        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        println!("Client got message '{}'. ", msg);
        let event: E = E::deserialize(msg.to_string()).unwrap();
        let old_state = Arc::clone(&self.state_holder.read().unwrap().get_state());
        let new_state = self.reducer.reduce(old_state, event);
        self.state_holder.write().unwrap().set_state(new_state);
        Ok(())
    }
}
