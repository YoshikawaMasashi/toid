extern crate ws;

use ws::connect;

fn main() {
    connect("ws://127.0.0.1:3012", |out| {
        out.send("Hello WebSocket").unwrap();

        move |msg| {
            println!("Got message: {}", msg);
            Ok(())
        }
    })
    .unwrap()
}
