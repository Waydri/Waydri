use super::client::WlClient;

pub struct ServerConnection {
    pub socket_name: String,
    pub listen_fd: i32,
    pub clients: Vec<WlClient>,
}

impl ServerConnection {
    pub fn new(socket_name: String) -> Self {
        ServerConnection {
            socket_name,
            listen_fd: -1,
            clients: Vec::new(),
        }
    }

    pub fn with_fd(socket_name: String, listen_fd: i32) -> Self {
        ServerConnection {
            socket_name,
            listen_fd,
            clients: Vec::new(),
        }
    }

    pub fn accept_client(&mut self, pid: u32, uid: u32) -> u32 {
        let id = (self.clients.len() + 1) as u32;
        self.clients.push(WlClient::new(id, self.listen_fd, pid, uid));
        id
    }

    pub fn drop_client(&mut self, id: u32) -> bool {
        let before = self.clients.len();
        self.clients.retain(|client| client.id != id);
        self.clients.len() != before
    }

    pub fn client(&self, id: u32) -> Option<&WlClient> {
        self.clients.iter().find(|client| client.id == id)
    }

    pub fn client_mut(&mut self, id: u32) -> Option<&mut WlClient> {
        self.clients.iter_mut().find(|client| client.id == id)
    }

    pub fn client_count(&self) -> usize {
        self.clients.len()
    }
}

pub struct WaylandServer {
    pub display_name: String,
    pub runtime_dir: String,
    pub connections: Vec<ServerConnection>,
    pub next_conn: u32,
}

impl WaylandServer {
    pub fn new(display_name: String, runtime_dir: String) -> Self {
        WaylandServer {
            display_name,
            runtime_dir,
            connections: Vec::new(),
            next_conn: 1,
        }
    }

    pub fn display_path(&self) -> String {
        format!("{}/{}", self.runtime_dir, self.display_name)
    }

    pub fn add_connection(&mut self, socket_name: String) -> u32 {
        let id = self.next_conn;
        self.next_conn += 1;
        self.connections.push(ServerConnection::new(socket_name));
        id
    }

    pub fn active_clients(&self) -> usize {
        self.connections.iter().map(|c| c.client_count()).sum()
    }
}