use std::collections::HashSet;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

#[allow(dead_code)]
pub(crate) struct P2PNode {
    listener: TcpListener,
    peers: Arc<Mutex<HashSet<String>>>,
}

#[allow(dead_code)]
impl P2PNode {
    pub(crate) fn new(addr: &str) -> std::io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;
        Ok(P2PNode {
            listener,
            peers: Arc::new(Mutex::new(HashSet::new())),
        })
    }

    pub(crate) fn start(&self) {
        let listener = self.listener.try_clone().unwrap();
        let peers = self.peers.clone();

        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let peers = peers.clone();
                        thread::spawn(move || {
                            handle_incoming(stream, peers);
                        });
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(100));
                    }
                    Err(e) => eprintln!("Connection failed: {}", e),
                }
            }
        });
    }

    pub(crate) fn connect(&self, addr: &str) {
        let addr = addr.to_string();

        thread::spawn(move || {
            loop {
                match TcpStream::connect(&addr) {
                    Ok(mut stream) => {
                        println!("Connected to {}", addr);

                        let local_addr = stream.local_addr().unwrap().to_string();
                        stream
                            .write_all(format!("PEER {}", local_addr).as_bytes())
                            .unwrap();

                        let mut buffer = [0; 1024];
                        loop {
                            match stream.read(&mut buffer) {
                                Ok(0) => break,
                                Ok(n) => {
                                    let msg = String::from_utf8_lossy(&buffer[..n]);
                                    println!("Received: {}", msg);
                                }
                                Err(_) => break,
                            }
                        }
                    }
                    Err(e) => eprintln!("Connection to {} failed: {}", addr, e),
                }
                thread::sleep(Duration::from_secs(5));
            }
        });
    }
}

fn handle_incoming(mut stream: TcpStream, peers: Arc<Mutex<HashSet<String>>>) {
    let addr = stream.peer_addr().unwrap().to_string();
    println!("New connection from: {}", addr);

    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                let msg = String::from_utf8_lossy(&buffer[..n]);
                if msg.starts_with("PEER ") {
                    let peer_addr = msg.trim_start_matches("PEER ").to_string();
                    peers.lock().unwrap().insert(peer_addr);
                }
                stream.write_all(b"PONG").unwrap();
            }
            Err(e) => {
                eprintln!("Error reading from {}: {}", addr, e);
                break;
            }
        }
    }

    println!("Connection closed: {}", addr);
    peers.lock().unwrap().remove(&addr);
}

/*fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // 检查参数数量
    if args.len() < 2 {
        eprintln!("Usage: {} <listen_address> [connect_address]", args[0]);
        std::process::exit(1);
    }

    let listen_addr = &args[1];
    let node = p2p_node::P2PNode::new(listen_addr)?;
    node.start();

    // 如果有第二个参数，连接指定节点
    if args.len() >= 3 {
        let connect_addr = &args[2];
        node.connect(connect_addr);
    }

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}*/
