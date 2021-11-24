use colored::Colorize;

use std::fs;
use std::io::prelude::*;
use std::net::{IpAddr, TcpStream};
use std::sync::{Arc, Mutex};

use crate::serialization::{Message, MessageContent};
use crate::shared_state::User;

pub struct Parser {
    pub user: Arc<Mutex<User>>,
}

impl Parser {
    pub fn new(user: Arc<Mutex<User>>) -> Parser {
        Parser { user }
    }

    pub fn parse_input(&self, input: &String) {
        if &input[0..1] == "/" {
            self.handle_command(&input[1..input.len() - 1]);
        } else {
            self.handle_text_message(&input[..input.len() - 1]);
        }
    }

    fn handle_command(&self, command: &str) {
        let tokens: Vec<&str> = command.split(" ").collect();
        match tokens[0] {
            "connect" => {
                if tokens.len() != 3 {
                    println!("{}", "Usage: /connect [ip] [port]".red());
                } else {
                    let ip: IpAddr = tokens[1].parse().unwrap();
                    let port: u16 = tokens[2].parse().unwrap();
                    let user = self.user.clone();
                    let mut user_guard = user.lock().unwrap();
                    user_guard.connected_to.push((ip, port));
                    println!("{}", format!("Connected to: {}:{}", ip, port).green());
                }
            }
            "set_username" => {
                if tokens.len() != 2 {
                    println!("{}", "Usage: /set_username [new_username]".red());
                } else {
                    if tokens[1].len() > 255 {
                        println!("{}", "Username Too Long".red());
                    } else {
                        let user = self.user.clone();
                        let mut user_guard = user.lock().unwrap();
                        user_guard.username = String::from(tokens[1]);
                        println!("{}", format!("Changed username to: {}", tokens[1]).green());
                    }
                }
            }
            "send_file" => {
                if tokens.len() != 3 {
                    println!(
                        "{}",
                        "Usage: /send_file [path_to_file] [name_to_send_file_as]".red()
                    );
                } else {
                    if tokens[2].len() > 255 {
                        println!("{}", "Name Too Long".red());
                    } else {
                        self.handle_file_message(tokens[1], tokens[2]);
                    }
                }
            }
            "exit" => {
                // TODO: Cleanly shut down
                println!("{}", "Exiting...".green());
                std::process::exit(0);
            }
            _ => {
                println!("{}", format!("Unrecognized Command: {}", tokens[0]).red());
            }
        };
    }

    fn handle_text_message(&self, message: &str) {
        let content = MessageContent::Text(String::from(message));
        self.handle_message(content);
    }

    fn handle_file_message(&self, path: &str, name: &str) {
        let name_length = name.len() as u8;
        let data = fs::read(path).unwrap();
        self.handle_message(MessageContent::File(name_length, String::from(name), data));
    }

    fn handle_message(&self, message: MessageContent) {
        let user = self.user.clone();
        let user_guard = user.lock().unwrap();
        let username = &user_guard.username;
        let packet = Message {
            sender: String::from(username),
            sender_length: username.len() as u8,
            content: message,
        };
        let packet_bytes = &packet.to_bytes();
        for ip in &user_guard.connected_to {
            let stream = TcpStream::connect(ip);
            match stream {
                Ok(mut stream) => {
                    let result = stream.write(packet_bytes);
                    if let Err(err_message) = result {
                        println!(
                            "{}",
                            format!(
                                "Error sending message to {}:{}: {}",
                                ip.0, ip.1, err_message
                            )
                            .red()
                        );
                    }
                }
                Err(err_message) => {
                    println!(
                        "{}",
                        format!("Error connecting to {}:{}: {}", ip.0, ip.1, err_message).red()
                    );
                }
            }
        }
    }
}
