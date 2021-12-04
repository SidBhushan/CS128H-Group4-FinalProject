mod encryption;
mod parsing;
mod serialization;
mod server;
mod shared_state;

use std::io;
use std::sync::{Arc, Mutex};
use std::thread;

use parsing::Parser;
use server::Server;
use shared_state::User;

fn main() {
    let user = Arc::new(Mutex::new(User::new()));

    let parser = Parser::new(user.clone());
    let input_parsing_thread = thread::spawn(move || {
        let mut input = String::new();
        loop {
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
            parser.parse_input(&input);
            input.clear();
        }
    });

    let server = Server::new(user.clone());
    let server_thread = thread::spawn(move || {
        for stream in server.listener.incoming() {
            let stream = stream.unwrap();
            server.handle_connection(stream);
        }
    });

    input_parsing_thread.join().unwrap();
    server_thread.join().unwrap();
}
