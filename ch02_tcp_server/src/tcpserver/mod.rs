use std::net::TcpListener;
use tracing::info;

pub fn run(port: u32) {
    let connect_listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    info!("running on port: {}", port);

    for stream in connect_listener.incoming() {
        let _stream = stream.unwrap();
        info!("connect established");
    }
}
