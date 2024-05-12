use std::net::TcpStream;
// use tracing::info;

pub fn run(port: u32) {
    let _stream = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
}
