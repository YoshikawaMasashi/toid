use toid::players::websocket_player::WebSocketPlayerServer;

fn main() {
    let mut ip = String::new();
    println!("please input ip (ex. 127.0.0.1):");
    std::io::stdin().read_line(&mut ip).unwrap();
    println!("ip: {}", ip);
    let ip = ip;
    let connect_address = format!("{}:3012", ip).replace("\n", "");
    println!("connect_address: ws://{}", connect_address);

    let mut server = WebSocketPlayerServer::new();
    server.listen(connect_address, Some("password".to_string()), None, Some(3));
}
