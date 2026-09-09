use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

pub struct IpcServer {
    pub path: String,
    pub handle: Option<JoinHandle<()>>,
    pub running: Arc<Mutex<bool>>,
    clients: Arc<Mutex<Vec<TcpStream>>>,
}

impl IpcServer {
    pub fn new(path: String) -> Self {
        IpcServer {
            path,
            handle: None,
            running: Arc::new(Mutex::new(false)),
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn start(&mut self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.path.clone())?;
        listener.set_nonblocking(true)?;
        *self.running.lock().unwrap() = true;
        let running = Arc::clone(&self.running);
        let clients = Arc::clone(&self.clients);
        let handle = std::thread::spawn(move || {
            while *running.lock().unwrap() {
                match listener.accept() {
                    Ok((stream, _)) => {
                        let _ = stream.set_nonblocking(true);
                        clients.lock().unwrap().push(stream);
                    }
                    Err(ref error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        std::thread::sleep(std::time::Duration::from_millis(20));
                    }
                    Err(_) => break,
                }
                let mut stale = Vec::new();
                {
                    let mut guard = clients.lock().unwrap();
                    guard.retain(|stream| stream.peer_addr().is_ok());
                    for stream in guard.iter_mut() {
                        let mut buffer = [0u8; 512];
                        match stream.read(&mut buffer) {
                            Ok(0) => {
                                stale.push(stream.local_addr().ok());
                                let _ = stream.shutdown(Shutdown::Both);
                            }
                            Ok(_) => {
                                let _ = stream.flush();
                            }
                            Err(ref error)
                                if error.kind() == std::io::ErrorKind::WouldBlock => {}
                            Err(_) => {
                                stale.push(stream.local_addr().ok());
                            }
                        }
                    }
                }
                {
                    let mut guard = clients.lock().unwrap();
                    guard.retain(|stream| {
                        stream
                            .local_addr()
                            .map(|addr| !stale.contains(&Some(addr)))
                            .unwrap_or(true)
                    });
                }
            }
        });
        self.handle = Some(handle);
        Ok(())
    }

    pub fn stop(&mut self) {
        *self.running.lock().unwrap() = false;
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
        self.clients.lock().unwrap().clear();
        let _ = std::fs::remove_file(&self.path);
    }

    pub fn is_running(&self) -> bool {
        *self.running.lock().unwrap()
    }
}

impl Drop for IpcServer {
    fn drop(&mut self) {
        self.stop();
    }
}

pub struct IpcClient {
    stream: TcpStream,
}

impl IpcClient {
    pub fn connect(path: &str) -> std::io::Result<Self> {
        let stream = TcpStream::connect(path)?;
        Ok(IpcClient { stream })
    }

    pub fn send_command(&mut self, payload: &str) -> std::io::Result<Vec<u8>> {
        self.stream.write_all(payload.as_bytes())?;
        self.stream.flush()?;
        let mut buffer = Vec::new();
        self.stream.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    pub fn close(self) {
        let _ = self.stream.shutdown(Shutdown::Both);
    }
}