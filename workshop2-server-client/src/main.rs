use std::net::{TcpListener, TcpStream};



struct ServiceConfig {
    port: u16,
    address: String,
}

struct Connection {
    id: u32,
    address: String,
    port: u16,
}

struct MainService {
    config: ServiceConfig,
    connections: Vec<Connection>,
}

fn handle_client(stream: TcpStream) {
    println!("New client connected");
}

#[tokio::main]
async fn main() {
    let config = ServiceConfig {
        port: 9999,
        address: "0.0.0.0".to_string(),
    };

    let service = MainService {
        config,
        connections: Vec::new(),
    };

    let listener = TcpListener::bind(format!("{}:{}", service.config.address, service.config.port)).unwrap();
    listener.set_nonblocking(true).unwrap();
     // accept connections and process them serially
     for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
    println!("Server is running on {}:{}", service.config.address, service.config.port);
    println!("Press Ctrl+C to stop the server");
}