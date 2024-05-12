use std::str;
use std::{
    io::{Read, Write},
    net::TcpStream,
};
use tracing::info;

pub fn run(port: u32) {
    let mut stream = TcpStream::connect(format!("localhost:{}", port)).unwrap();
    info!("connecting to port: {}", port);

    stream.write("Hello".as_bytes()).unwrap();
    stream.flush().unwrap();

    let mut buffer = [0; 7];
    stream.read(&mut buffer).unwrap();

    info!(
        "got response from server: {:?}",
        str::from_utf8(&buffer).unwrap()
    );
}
