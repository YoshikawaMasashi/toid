use ws;

pub struct WebSocketPlayerServer {}

pub struct WebSocketPlayerServerHandler {
    out: ws::Sender,
}

impl WebSocketPlayerServer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn listen(&mut self, connect_address: String) {
        if let Err(error) = ws::listen(connect_address, |out| {
            WebSocketPlayerServerHandler::new(out)
        }) {
            println!("Failed to create WebSocket due to {:?}", error);
        }
    }
}

impl WebSocketPlayerServerHandler {
    fn new(out: ws::Sender) -> Self {
        Self { out }
    }
}

impl ws::Handler for WebSocketPlayerServerHandler {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        println!("Server got message '{}'. ", msg);

        self.out.broadcast(msg)
    }
}
