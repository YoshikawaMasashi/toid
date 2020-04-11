use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::RwLock;

use ws;

use super::super::super::state_management::reducer::Reducer;
use super::super::super::state_management::serialize::Serialize;
use super::super::super::state_management::state_holder::StateHolder;
use super::super::super::state_management::store::Store;

pub struct WebSocketStore<S, E, R> {
    out: Option<Arc<ws::Sender>>,
    state_holder: Arc<RwLock<StateHolder<S>>>,
    event_marker: PhantomData<E>,
    reducer: Arc<R>,
}

pub struct WebSocketStoreHandler<S, E, R> {
    state_holder: Arc<RwLock<StateHolder<S>>>,
    reducer: Arc<R>,
    event_marker: PhantomData<E>,
}

impl<S, E: Sized + Serialize<E>, R: Reducer<S, E>> WebSocketStore<S, E, R> {
    pub fn new(state: S, reducer: R) -> Self {
        Self {
            out: None,
            state_holder: Arc::new(RwLock::new(StateHolder::new(state))),
            event_marker: PhantomData,
            reducer: Arc::new(reducer),
        }
    }

    pub fn connect(&mut self, connect_address: String) {
        if let Err(error) = ws::connect(connect_address, |out| {
            let out_arc = Arc::new(out);
            self.out = Some(Arc::clone(&out_arc));
            let handler: WebSocketStoreHandler<S, E, R> = WebSocketStoreHandler {
                state_holder: Arc::clone(&self.state_holder),
                reducer: Arc::clone(&self.reducer),
                event_marker: PhantomData,
            };
            handler
        }) {
            println!("Failed to create WebSocket due to: {:?}", error);
        }
    }
}

impl<S, E: Sized + Serialize<E>, R> Store<S, E> for WebSocketStore<S, E, R> {
    fn get_state(&self) -> Arc<S> {
        Arc::clone(&self.state_holder.read().unwrap().get_state())
    }

    fn update_state(&self, event: E) {
        if let Some(out) = &self.out {
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
