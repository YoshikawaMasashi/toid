use toid::reducers::websocket_reducer::WebSocketReducerServer;

fn main() {
    let server = WebSocketReducerServer::new();
    server.run();
}
