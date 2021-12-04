use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::{Arc, Mutex};

use colored::Colorize;

use crate::serialization::{Message, MessageContent};
use crate::shared_state::User;

pub struct Server {
    pub port: u16,
    pub listener: TcpListener,
    pub user: Arc<Mutex<User>>,
}

impl Server {
    pub fn new(user: Arc<Mutex<User>>) -> Server {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let user_copy = user.clone();
        let address = listener.local_addr().unwrap();
        user_copy.lock().unwrap().username = format!("User@{}", address);
        println!(
            "{}",
            format!("Started at ip {} on port {}", address.ip(), address.port()).yellow()
        );
        Server {
            port: address.port(),
            listener,
            user,
        }
    }

    pub fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer: Vec<u8> = Vec::new();
        stream.read_to_end(&mut buffer).unwrap();

        let user = self.user.clone();
        let user_guard = user.lock().unwrap();
        let encryptor = &user_guard.encryptor;
        encryptor.apply_encryption(&mut buffer);

        let message = Message::from_bytes(&buffer);
        match message.content {
            MessageContent::Text(_) => {
                Server::handle_text_message(message);
            }
            MessageContent::File(_, _, _) => {
                Server::handle_file_message(message);
            }
        };
    }

    fn handle_text_message(message: Message) {
        if let MessageContent::Text(content) = message.content {
            println!("{}: {}", message.sender.purple(), content);
        }
    }

    fn handle_file_message(message: Message) {
        if let MessageContent::File(_, name, data) = message.content {
            let dir_name = String::from("./files/");
            let dir_path = Path::new(&dir_name);
            if !dir_path.exists() {
                let create_dir = fs::create_dir(dir_path);
                if let Err(error) = create_dir {
                    println!("{}", format!("Error Saving File: {}", error).red());
                }
            }
            let path = dir_path.join(&name);
            let file_write = fs::write(path, data);
            match file_write {
                Ok(_) => {
                    println!(
                        "{}: File saved to {}",
                        message.sender.purple(),
                        dir_name + &name
                    );
                }
                Err(error) => {
                    println!("{}", format!("Error Saving File: {}", error).red());
                }
            }
        }
    }
}
