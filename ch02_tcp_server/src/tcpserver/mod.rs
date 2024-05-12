use std::{io::Read, io::Write, net::TcpListener};
use tracing::info;

pub fn run(port: u32) {
    let connect_listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    info!("running on port: {}", port);

    for stream in connect_listener.incoming() {
        match stream {
            Ok(mut stream) => {
                info!("connect established");
                let mut buffer = [0; 1024];
                stream.read(&mut buffer).unwrap();
                stream.write(&mut buffer).unwrap();
            }
            Err(e) => {
                info!("connection failed: {}", e)
            }
        }
    }
}
