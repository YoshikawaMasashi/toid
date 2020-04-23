use std::borrow::Cow;
use std::marker::PhantomData;
use std::marker::{Send, Sync};
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

use openssl::ssl::{SslConnector, SslMethod, SslStream, SslVerifyMode};
use url;
use ws;
use ws::util::TcpStream;

use super::super::super::resource_management::resource_manager::{
    ResourceManager, ResourceManagerEvent,
};
use super::super::super::state_management::serialize::Serialize;
use super::super::super::state_management::state::State;
use super::super::super::state_management::store::Store;
use super::super::super::state_management::store_reader::StoreReader;
use super::super::player::Player;
use super::send_data::SendData;

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

pub struct WebSocketPlayer<S, E, R, O, RE> {
    store: Arc<Store<S, E>>,
    resource_manager: Arc<ResourceManager>,
    sender_holder: Arc<RwLock<SenderHolder>>,
    reader: Arc<RwLock<R>>,
    out_marker: PhantomData<O>,
    reader_event_marker: PhantomData<RE>,
}

pub struct WebSocketPlayerHandler<S, E, R, O, RE> {
    store: Arc<Store<S, E>>,
    reader: Arc<RwLock<R>>,
    out_marker: PhantomData<O>,
    reader_event_marker: PhantomData<RE>,
    resource_manager: Arc<ResourceManager>,
}

impl<
        S: 'static + State<E> + Serialize<S> + Send + Sync,
        E: 'static + Sized + Serialize<E> + Send + Sync,
        R: 'static + StoreReader<O, RE, S, E> + Send + Sync,
        O,
        RE: 'static + Sized + Serialize<RE> + Send + Sync,
    > WebSocketPlayer<S, E, R, O, RE>
{
    pub fn new() -> Self {
        Self {
            store: Arc::new(Store::new(S::new())),
            resource_manager: Arc::new(ResourceManager::new()),
            reader: Arc::new(RwLock::new(R::new())),
            sender_holder: Arc::new(RwLock::new(SenderHolder::new())),
            out_marker: PhantomData,
            reader_event_marker: PhantomData,
        }
    }

    pub fn connect(&mut self, connect_address: String) {
        let store = Arc::clone(&self.store);
        let sender_holder = Arc::clone(&self.sender_holder);
        let reader = Arc::clone(&self.reader);
        let resource_manager = Arc::clone(&self.resource_manager);
        thread::spawn(move || {
            if let Err(error) = ws::connect(connect_address, |out| {
                sender_holder.write().unwrap().set_sender(out); // TODO: remove unwrap
                let handler: WebSocketPlayerHandler<S, E, R, O, RE> = WebSocketPlayerHandler {
                    store: Arc::clone(&store),
                    reader: Arc::clone(&reader),
                    out_marker: PhantomData,
                    reader_event_marker: PhantomData,
                    resource_manager: Arc::clone(&resource_manager),
                };
                handler
            }) {
                println!("Failed to create WebSocket due to: {:?}", error);
            }
        });
    }

    pub fn sync_state(&self) -> Result<(), String> {
        match &self.sender_holder.read().map_err(|_| "rwlock error")?.out {
            Some(out) => {
                let serialized_state = self.store.get_state()?.serialize()?;
                let msg = SendData::SyncState(serialized_state).serialize()?;
                out.send(msg).map_err(|e| e.to_string())?;
                Ok(())
            }
            None => Err("sender have not been prepared yet".to_string()),
        }
    }
}

impl<
        S: State<E>,
        E: Sized + Serialize<E>,
        R: StoreReader<O, RE, S, E>,
        O,
        RE: Sized + Serialize<RE>,
    > Player<S, E, R, O, RE> for WebSocketPlayer<S, E, R, O, RE>
{
    fn get_store(&self) -> Arc<Store<S, E>> {
        Arc::clone(&self.store)
    }

    fn get_resource_manager(&self) -> Arc<ResourceManager> {
        Arc::clone(&self.resource_manager)
    }

    fn get_reader(&self) -> Arc<RwLock<R>> {
        Arc::clone(&self.reader)
    }

    fn send_event(&self, event: E) -> Result<(), String> {
        match &self.sender_holder.read().map_err(|_| "rwlock error")?.out {
            Some(out) => {
                let serialized_event = event.serialize()?;
                let msg = SendData::StateUpdate(serialized_event).serialize()?;
                out.send(msg).map_err(|e| e.to_string())?;
                Ok(())
            }
            None => Err("sender have not been prepared yet".to_string()),
        }
    }

    fn send_reader_event(&self, event: RE) -> Result<(), String> {
        match &self.sender_holder.read().map_err(|_| "rwlock error")?.out {
            Some(out) => {
                let serialized_event = event.serialize()?;
                let msg = SendData::ApplyReader(serialized_event).serialize()?;
                out.send(msg).map_err(|e| e.to_string())?;
                Ok(())
            }
            None => Err("sender have not been prepared yet".to_string()),
        }
    }

    fn send_resource_event(&self, event: ResourceManagerEvent) -> Result<(), String> {
        match &self.sender_holder.read().map_err(|_| "rwlock error")?.out {
            Some(out) => {
                let serialized_event = event.serialize()?;
                let msg = SendData::ApplyResourceManager(serialized_event).serialize()?;
                out.send(msg).map_err(|e| e.to_string())?;
                Ok(())
            }
            None => Err("sender have not been prepared yet".to_string()),
        }
    }
}

impl<
        S: State<E> + Serialize<S>,
        E: Sized + Serialize<E>,
        R: StoreReader<O, RE, S, E>,
        O,
        RE: Sized + Serialize<RE>,
    > ws::Handler for WebSocketPlayerHandler<S, E, R, O, RE>
{
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        println!("connected");
        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        println!("Client got message '{}'. ", msg);
        let send_data: SendData =
            SendData::deserialize(msg.to_string()).map_err(|e| ws::Error {
                kind: ws::ErrorKind::Internal,
                details: Cow::from(e),
            })?;
        match send_data {
            SendData::StateUpdate(event_string) => {
                let event: E = E::deserialize(event_string).map_err(|e| ws::Error {
                    kind: ws::ErrorKind::Internal,
                    details: Cow::from(e),
                })?;
                self.store.update_state(event).map_err(|e| ws::Error {
                    kind: ws::ErrorKind::Internal,
                    details: Cow::from(e),
                })?;
                Ok(())
            }
            SendData::SyncState(state_string) => {
                let state: S = S::deserialize(state_string).map_err(|e| ws::Error {
                    kind: ws::ErrorKind::Internal,
                    details: Cow::from(e),
                })?;
                self.store.set_state(state).map_err(|e| ws::Error {
                    kind: ws::ErrorKind::Internal,
                    details: Cow::from(e),
                })?;
                Ok(())
            }
            SendData::ApplyReader(event_string) => {
                let event: RE = RE::deserialize(event_string).map_err(|e| ws::Error {
                    kind: ws::ErrorKind::Internal,
                    details: Cow::from(e),
                })?;
                self.reader
                    .write()
                    .map_err(|_| ws::Error {
                        kind: ws::ErrorKind::Internal,
                        details: Cow::from("RwLock Error"),
                    })?
                    .apply(event);
                Ok(())
            }
            SendData::ApplyResourceManager(event_string) => {
                let event: ResourceManagerEvent = ResourceManagerEvent::deserialize(event_string)
                    .map_err(|e| ws::Error {
                    kind: ws::ErrorKind::Internal,
                    details: Cow::from(e),
                })?;
                if let Err(error) = self.resource_manager.apply(event) {
                    println!("send resource event error !: {}", error);
                }
                Ok(())
            }
        }
    }

    fn upgrade_ssl_client(
        &mut self,
        sock: TcpStream,
        _: &url::Url,
    ) -> ws::Result<SslStream<TcpStream>> {
        let mut builder = SslConnector::builder(SslMethod::tls()).map_err(|e| {
            ws::Error::new(
                ws::ErrorKind::Internal,
                format!("Failed to upgrade client to SSL: {}", e),
            )
        })?;
        builder.set_verify(SslVerifyMode::empty());

        let connector = builder.build();
        connector
            .configure()
            .unwrap()
            .use_server_name_indication(false)
            .verify_hostname(false)
            .connect("", sock)
            .map_err(From::from)
    }
}
