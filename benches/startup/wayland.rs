use std::time::Instant;

use waydri_core::wayland::client::WlClient;
use waydri_core::wayland::server::{ServerConnection, WaylandServer};
use waydri_core::wayland::{MessageCodec, WaylandMessage};

fn main() {
    let start = Instant::now();
    let mut server = WaylandServer::new("wayland-0".to_string(), "/tmp".to_string());
    let connection_id = server.add_connection("wayland-0".to_string());
    let elapsed = start.elapsed();
    println!("server init: {:.3} ms", elapsed.as_secs_f64() * 1000.0);
    assert_eq!(server.display_path(), "/tmp/wayland-0");

    let mut connection = ServerConnection::with_fd("wayland-0".to_string(), 3);
    let start = Instant::now();
    let mut ids = Vec::new();
    for _ in 0..100_000 {
        ids.push(connection.accept_client(100, 2000));
    }
    let elapsed = start.elapsed();
    println!("accept: {:.0} clients/s", 100_000.0 / elapsed.as_secs_f64());
    assert_eq!(connection.client_count(), 100_000);

    let start = Instant::now();
    for id in ids {
        connection.drop_client(id);
    }
    let elapsed = start.elapsed();
    println!("drop: {:.0} clients/s", 100_000.0 / elapsed.as_secs_f64());
    assert_eq!(connection.client_count(), 0);

    let mut client = WlClient::new(1, 1, 100, 2000);
    client.add_object(2);
    let message = WaylandMessage::new(1, 0, vec![]);
    let encoded = MessageCodec::encode(&message);
    assert_eq!(encoded.len(), 8);
    let _ = connection_id;
}