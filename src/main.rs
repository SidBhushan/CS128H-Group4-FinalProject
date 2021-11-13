mod parsing;
mod serialization;
mod server;

use std::io;
use std::thread;

use server::Server;

fn main() {
    let input_parsing_thread = thread::spawn(|| {
        let mut input = String::new();
        loop {
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
            parsing::parse_input(&input);
            input.clear();
        }
    });

    let server_thread = thread::spawn(|| {
        let server = Server::new();
        println!("Server started on port: {}", server.port);
        for stream in server.listener.incoming() {
            println!("Received stream");
            let stream = stream.unwrap();
            Server::handle_connection(stream);
        }
    });

    input_parsing_thread.join().unwrap();
    server_thread.join().unwrap();
}
