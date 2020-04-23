use std::borrow::Cow;
use std::sync::Arc;
use std::sync::RwLock;

use base64;
use openssl::ssl::{SslAcceptor, SslStream};
use ws;
use ws::util::TcpStream;

pub struct WebSocketPlayerServer {
    connection_counter: Arc<RwLock<ConnectionCounter>>,
}

struct WebSocketPlayerServerHandler {
    out: ws::Sender,
    password: Option<String>,
    ssl: Option<Arc<SslAcceptor>>,
    max_connection: Option<usize>,
    connection_counter: Arc<RwLock<ConnectionCounter>>,
}

struct ConnectionCounter {
    num: usize,
}

impl ConnectionCounter {
    fn increment(&mut self) {
        self.num += 1;
    }

    fn decrement(&mut self) {
        self.num -= 1;
    }

    fn get_num(&self) -> usize {
        self.num
    }
}

impl WebSocketPlayerServer {
    pub fn new() -> Self {
        Self {
            connection_counter: Arc::new(RwLock::new(ConnectionCounter { num: 0 })),
        }
    }

    pub fn listen(
        &mut self,
        connect_address: String,
        password: Option<String>,
        ssl: Option<Arc<SslAcceptor>>,
        max_connection: Option<usize>,
    ) {
        let connection_counter = Arc::clone(&self.connection_counter);
        if let Err(error) = ws::listen(connect_address, move |out| {
            WebSocketPlayerServerHandler::new(
                out,
                password.clone(),
                ssl.clone(),
                max_connection.clone(),
                Arc::clone(&connection_counter),
            )
        }) {
            println!("Failed to create WebSocket due to {:?}", error);
        }
    }
}

impl WebSocketPlayerServerHandler {
    fn new(
        out: ws::Sender,
        password: Option<String>,
        ssl: Option<Arc<SslAcceptor>>,
        max_connection: Option<usize>,
        connection_counter: Arc<RwLock<ConnectionCounter>>,
    ) -> Self {
        Self {
            out,
            password,
            ssl,
            max_connection,
            connection_counter,
        }
    }
}

impl ws::Handler for WebSocketPlayerServerHandler {
    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        println!("on open");
        println!("connection id : {}", self.out.connection_id());
        self.connection_counter.write().unwrap().increment();

        println!(
            "connection num: {}",
            self.connection_counter.read().unwrap().get_num()
        );

        match self.max_connection {
            Some(max_connection) => {
                if self.connection_counter.read().unwrap().get_num() > max_connection {
                    self.out
                        .close_with_reason(ws::CloseCode::Error, "over max connection")?;
                }
            }
            None => {}
        };

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

    fn on_close(&mut self, _: ws::CloseCode, reason: &str) {
        self.connection_counter.write().unwrap().decrement();
        println!(
            "Closed WebSocket. reason: {}, connection_id: {}",
            reason,
            self.out.connection_id()
        );
        println!(
            "connection num: {}",
            self.connection_counter.read().unwrap().get_num()
        );
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
