use std::net::IpAddr;

pub struct User {
    pub username: String,
    pub connected_to: Vec<(IpAddr, u16)>,
}

impl User {
    pub fn new() -> User {
        User {
            username: String::from(""),
            connected_to: Vec::new(),
        }
    }
}
