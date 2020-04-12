use std::marker::{Send, Sync};
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

use ws;

use super::super::super::resource_management::resource_manager::ResourceManager;
use super::super::super::state_management::serialize::Serialize;
use super::super::super::state_management::state::State;
use super::super::super::state_management::store::Store;
use super::super::player::Player;

pub struct SenderHolder {
    out: Option<ws::Sender>,
}

impl SenderHolder {
    fn new() -> Self {
        Self { out: None }
    }

    fn set_sender(&mut self, out: ws::Sender) {
        self.out = Some(out);
    }
}

pub struct WebSocketPlayer<S, E> {
    store: Arc<Store<S, E>>,
    resource_manager: Arc<ResourceManager>,
    sender_holder: Arc<RwLock<SenderHolder>>,
}

pub struct WebSocketPlayerHandler<S, E> {
    store: Arc<Store<S, E>>,
}

impl<S: 'static + State<E> + Send + Sync, E: 'static + Sized + Serialize<E> + Send + Sync>
    WebSocketPlayer<S, E>
{
    pub fn new(store: Arc<Store<S, E>>, resource_manager: Arc<ResourceManager>) -> Self {
        Self {
            store: store,
            resource_manager: resource_manager,
            sender_holder: Arc::new(RwLock::new(SenderHolder::new())),
        }
    }

    pub fn connect(&mut self, connect_address: String) {
        let store = Arc::clone(&self.store);
        let sender_holder = Arc::clone(&self.sender_holder);
        thread::spawn(move || {
            if let Err(error) = ws::connect(connect_address, |out| {
                sender_holder.write().unwrap().set_sender(out);
                let handler: WebSocketPlayerHandler<S, E> = WebSocketPlayerHandler {
                    store: Arc::clone(&store),
                };
                handler
            }) {
                println!("Failed to create WebSocket due to: {:?}", error);
            }
        });
    }
}

impl<S: State<E>, E: Sized + Serialize<E>> Player<S, E> for WebSocketPlayer<S, E> {
    fn get_store(&self) -> Arc<Store<S, E>> {
        Arc::clone(&self.store)
    }

    fn get_resource_manager(&self) -> Arc<ResourceManager> {
        Arc::clone(&self.resource_manager)
    }

    fn send_event(&self, event: E) {
        if let Some(out) = &self.sender_holder.read().unwrap().out {
            let serialized = event.serialize().unwrap();
            out.send(serialized).unwrap();
        } else {
            println!("sender have not been prepared yet");
        }
    }
}

impl<S: State<E>, E: Sized + Serialize<E>> ws::Handler for WebSocketPlayerHandler<S, E> {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        println!("connected");
        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        println!("Client got message '{}'. ", msg);
        let event: E = E::deserialize(msg.to_string()).unwrap();
        self.store.update_state(event);
        Ok(())
    }
}
