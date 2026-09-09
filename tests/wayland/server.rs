use waydri_core::wayland::client::WlClient;
use waydri_core::wayland::server::{ServerConnection, WaylandServer};

#[test]
fn server_creates_display_path() {
    let server = WaylandServer::new("wayland-0".to_string(), "/tmp".to_string());
    assert_eq!(server.display_path(), "/tmp/wayland-0");
}

#[test]
fn add_connection_is_seq() {
    let mut server = WaylandServer::new("wayland-0".to_string(), "/tmp".to_string());
    let first = server.add_connection("wayland-0".to_string());
    let second = server.add_connection("wayland-1".to_string());
    assert!(second > first);
    assert_eq!(server.connections.len(), 2);
}

#[test]
fn connection_accepts_and_drops_clients() {
    let mut connection = ServerConnection::with_fd("wayland-0".to_string(), 3);
    let id = connection.accept_client(100, 2000);
    assert_eq!(connection.client_count(), 1);
    assert!(connection.client(id).is_some());
    assert!(connection.drop_client(id));
    assert_eq!(connection.client_count(), 0);
}

#[test]
fn active_clients_aggregates() {
    let mut server = WaylandServer::new("wayland-0".to_string(), "/tmp".to_string());
    let id = server.add_connection("wayland-0".to_string());
    let connection = &mut server.connections[id as usize - 1];
    connection.accept_client(1, 1);
    connection.accept_client(2, 1);
    assert_eq!(server.active_clients(), 2);
}

#[test]
fn client_mut_allows_mutation() {
    let mut connection = ServerConnection::with_fd("wayland-0".to_string(), 3);
    let id = connection.accept_client(1, 1);
    if let Some(client) = connection.client_mut(id) {
        client.add_object(3);
    }
    assert!(connection.client(id).unwrap().owns(3));
}

#[test]
fn default_connection_closed_fd() {
    let connection = ServerConnection::new("wayland-0".to_string());
    assert_eq!(connection.listen_fd, -1);
    assert_eq!(connection.client_count(), 0);
}

#[test]
fn dropping_unknown_client_fails() {
    let mut connection = ServerConnection::new("wayland-0".to_string());
    assert!(!connection.drop_client(99));
}

#[test]
fn wl_client_default_fd() {
    let client = WlClient::new(1, -1, 0, 0);
    assert_eq!(client.fd, -1);
}