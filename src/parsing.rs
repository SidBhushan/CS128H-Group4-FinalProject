use colored::Colorize;

pub fn parse_input(input: &String) {
    if &input[0..1] == "/" {
        handle_command(&input[1..input.len() - 1]);
    } else {
        handle_message(&input[..input.len() - 1]);
    }
}

fn handle_command(command: &str) {
    println!("Handing command: {}", command);
    let tokens: Vec<&str> = command.split(" ").collect();
    match tokens[0] {
        "send" => {
            use std::io::prelude::*;
            use std::net::TcpStream;
            let mut stream = TcpStream::connect(String::from("127.0.0.1:") + tokens[1]).unwrap();
            stream.write(tokens[2].as_bytes()).unwrap();
        }
        "exit" => {
            // TODO: Cleanly shut down
            println!("{}", "Exiting...".green());
            std::process::exit(0);
        }
        _ => {
            println!("{}", format!("Unrecognized Command: {}", tokens[0]).red());
        }
    }
}

fn handle_message(message: &str) {
    println!("Handling message: {} ({})", message, message.len());
}
