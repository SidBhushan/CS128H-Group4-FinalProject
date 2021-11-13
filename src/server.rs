use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

pub struct Server {
    pub port: u16,
    pub listener: TcpListener,
}

impl Server {
    pub fn new() -> Server {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        Server {
            port: listener.local_addr().unwrap().port(),
            listener,
        }
    }

    pub fn handle_connection(mut stream: TcpStream) {
        let mut buffer: Vec<u8> = Vec::new();
        stream.read_to_end(&mut buffer).unwrap();
        println!("Buffer size: {}", buffer.len());
        println!("Buffer contents: {}", String::from_utf8(buffer).unwrap());
    }
}
