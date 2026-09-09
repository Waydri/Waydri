use super::protocol::{INTERFACE_DISPLAY, WaylandMessage};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientState {
    Connected,
    Running,
    Idle,
    Detached,
}

#[derive(Debug)]
pub struct WlClient {
    pub id: u32,
    pub fd: i32,
    pub pid: u32,
    pub uid: u32,
    pub state: ClientState,
    pub objects: Vec<u32>,
    pub ping_serial: u32,
    pub ping_due: u64,
}

impl WlClient {
    pub fn new(id: u32, fd: i32, pid: u32, uid: u32) -> Self {
        WlClient {
            id,
            fd,
            pid,
            uid,
            state: ClientState::Connected,
            objects: Vec::new(),
            ping_serial: 0,
            ping_due: 0,
        }
    }

    pub fn add_object(&mut self, object: u32) {
        if !self.objects.contains(&object) {
            self.objects.push(object);
        }
    }

    pub fn remove_object(&mut self, object: u32) -> bool {
        let before = self.objects.len();
        self.objects.retain(|o| *o != object);
        self.objects.len() != before
    }

    pub fn owns(&self, object: u32) -> bool {
        self.objects.contains(&object)
    }

    pub fn schedule_ping(&mut self, serial: u32) {
        self.ping_serial = serial;
        self.ping_due = crate::utils::time::now_millis() as u64 + 5000;
    }

    pub fn ping_expired(&self) -> bool {
        self.ping_due != 0 && crate::utils::time::now_millis() as u64 > self.ping_due
    }

    pub fn dispatch(&self, message: &WaylandMessage) -> Option<Result<(), String>> {
        if message.object_id == 1 && message.opcode == super::protocol::OPCODE_SYNC {
            Some(Ok(()))
        } else if message.object_id == 1 && message.opcode == super::protocol::OPCODE_GET_REGISTRY {
            Some(Ok(()))
        } else {
            None
        }
    }
}

pub fn connect_display(_socket_name: &str) -> Result<i32, String> {
    Err("native display connection handled by the host runtime".to_string())
}

pub fn maybe_detach(read: &[u8]) -> ClientState {
    if read.windows(4).any(|window| window == INTERFACE_DISPLAY.as_bytes()) {
        ClientState::Idle
    } else {
        ClientState::Running
    }
}