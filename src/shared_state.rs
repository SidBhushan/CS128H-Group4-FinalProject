use std::net::IpAddr;

use crate::encryption::Encryptor;

pub struct User {
    pub username: String,
    pub connected_to: Vec<(IpAddr, u16)>,
    pub encryptor: Encryptor,
}

impl User {
    pub fn new() -> User {
        User {
            username: String::from(""),
            connected_to: Vec::new(),
            encryptor: Encryptor::new(),
        }
    }
}
