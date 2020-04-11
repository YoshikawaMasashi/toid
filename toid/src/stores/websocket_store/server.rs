use ws;

pub struct WebSocketStoreServer {}

pub struct WebSocketStoreServerHandler {
    out: ws::Sender,
}

impl WebSocketStoreServer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn listen(&mut self, connect_address: String) {
        if let Err(error) = ws::listen(connect_address, |out| WebSocketStoreServerHandler::new(out))
        {
            println!("Failed to create WebSocket due to {:?}", error);
        }
    }
}

impl WebSocketStoreServerHandler {
    fn new(out: ws::Sender) -> Self {
        Self { out }
    }
}

impl ws::Handler for WebSocketStoreServerHandler {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        println!("Server got message '{}'. ", msg);

        self.out.broadcast(msg)
    }
}
