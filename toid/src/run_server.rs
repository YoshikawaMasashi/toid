use ws::listen;

fn main() {
    listen("127.0.0.1:3012", |out| move |msg| out.broadcast(msg)).unwrap()
}
