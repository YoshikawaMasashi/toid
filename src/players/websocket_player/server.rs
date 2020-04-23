use std::borrow::Cow;
use std::sync::Arc;

use base64;
use openssl::ssl::{SslAcceptor, SslStream};
use ws;
use ws::util::TcpStream;

pub struct WebSocketPlayerServer {}

pub struct WebSocketPlayerServerHandler {
    out: ws::Sender,
    password: Option<String>,
    ssl: Option<Arc<SslAcceptor>>,
}

impl WebSocketPlayerServer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn listen(
        &mut self,
        connect_address: String,
        password: Option<String>,
        ssl: Option<Arc<SslAcceptor>>,
    ) {
        if let Err(error) = ws::listen(connect_address, |out| {
            WebSocketPlayerServerHandler::new(out, password.clone(), ssl.clone())
        }) {
            println!("Failed to create WebSocket due to {:?}", error);
        }
    }
}

impl WebSocketPlayerServerHandler {
    fn new(out: ws::Sender, password: Option<String>, ssl: Option<Arc<SslAcceptor>>) -> Self {
        Self { out, password, ssl }
    }
}

impl ws::Handler for WebSocketPlayerServerHandler {
    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        println!("on open");
        println!("connection id : {}", self.out.connection_id());

        match &self.password {
            Some(password) => match shake.request.header("Authorization".into()) {
                Some(auth_header) => {
                    let auth_header = auth_header.to_vec();
                    let auth_header = String::from_utf8(auth_header).unwrap();
                    let auth_header = auth_header.split(' ').last().unwrap();
                    let auth_header = base64::decode(auth_header).unwrap();
                    let auth_header = String::from_utf8(auth_header).unwrap();

                    let user_pass: Vec<&str> = auth_header.split(':').collect();
                    let user = user_pass[0];
                    let pass = user_pass[1];

                    println!("user:{}, pass:{}", user, pass);
                    if pass == password {
                        println!("good auth");
                    } else {
                        println!("bad auth");
                        self.out
                            .close_with_reason(ws::CloseCode::Error, "invalid passwd")?;
                    }
                }
                None => {
                    self.out
                        .close_with_reason(ws::CloseCode::Error, "please specify passwd")?;
                }
            },
            None => {}
        }
        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        println!("Server got message '{}'. ", msg);

        self.out.broadcast(msg)
    }

    fn upgrade_ssl_server(&mut self, sock: TcpStream) -> ws::Result<SslStream<TcpStream>> {
        match &self.ssl {
            Some(ssl) => ssl.accept(sock).map_err(From::from),
            None => Err(ws::Error {
                kind: ws::ErrorKind::Internal,
                details: Cow::from("not have ssl"),
            }),
        }
    }
}
