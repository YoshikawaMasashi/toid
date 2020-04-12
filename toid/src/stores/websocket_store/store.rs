use std::marker::PhantomData;
use std::marker::{Send, Sync};
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

use ws;

use super::super::super::state_management::serialize::Serialize;
use super::super::super::state_management::state::State;
use super::super::super::state_management::state_holder::StateHolder;
use super::super::super::state_management::store::Store;

pub struct SenderHolder {
    out: Option<ws::Sender>,
}

pub struct WebSocketStore<S, E> {
    sender_holder: Arc<RwLock<SenderHolder>>,
    state_holder: Arc<RwLock<StateHolder<S>>>,
    event_marker: PhantomData<E>,
}

pub struct WebSocketStoreHandler<S, E> {
    state_holder: Arc<RwLock<StateHolder<S>>>,
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

impl<S: 'static + State<E> + Send + Sync, E: Sized + Serialize<E> + Send + Sync>
    WebSocketStore<S, E>
{
    pub fn new(state: S) -> Self {
        Self {
            sender_holder: Arc::new(RwLock::new(SenderHolder::new())),
            state_holder: Arc::new(RwLock::new(StateHolder::new(state))),
            event_marker: PhantomData,
        }
    }

    pub fn connect(&mut self, connect_address: String) {
        let state_holder = Arc::clone(&self.state_holder);
        let sender_holder = Arc::clone(&self.sender_holder);
        thread::spawn(move || {
            if let Err(error) = ws::connect(connect_address, |out| {
                sender_holder.write().unwrap().set_sender(out);
                let handler: WebSocketStoreHandler<S, E> = WebSocketStoreHandler {
                    state_holder: Arc::clone(&state_holder),
                    event_marker: PhantomData,
                };
                handler
            }) {
                println!("Failed to create WebSocket due to: {:?}", error);
            }
        });
    }
}

impl<S, E: Sized + Serialize<E>> Store<S, E> for WebSocketStore<S, E> {
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

impl<S: State<E>, E: Sized + Serialize<E>> ws::Handler for WebSocketStoreHandler<S, E> {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        println!("connected");
        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        println!("Client got message '{}'. ", msg);
        let event: E = E::deserialize(msg.to_string()).unwrap();
        let old_state = Arc::clone(&self.state_holder.read().unwrap().get_state());
        let new_state = old_state.reduce(event);
        self.state_holder.write().unwrap().set_state(new_state);
        Ok(())
    }
}
