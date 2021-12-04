use colored::Colorize;

const INTEGRITY_BITS: [u8; 8] = [0, 31, 63, 95, 127, 159, 191, 223];
const INTEGRITY_BITS_LENGTH: usize = INTEGRITY_BITS.len();

pub struct Message {
    pub sender: String,
    pub sender_length: u8,
    pub content: MessageContent,
}

pub enum MessageContent {
    Text(String),
    File(u8, String, Vec<u8>),
}

impl Message {
    fn get_variant_id(&self) -> u8 {
        match &self.content {
            MessageContent::Text(_) => 0,
            MessageContent::File(_, _, _) => 1,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.append(&mut INTEGRITY_BITS.to_vec());
        result.push(self.sender_length);
        result.append(&mut self.sender.as_bytes().to_vec());
        result.push(self.get_variant_id());
        match &self.content {
            MessageContent::Text(data) => {
                result.append(&mut data.as_bytes().to_vec());
            }
            MessageContent::File(name_length, name, data) => {
                result.push(*name_length);
                result.append(&mut name.as_bytes().to_vec());
                result.append(&mut data.clone());
            }
        };
        result
    }

    pub fn from_bytes(bytes: &Vec<u8>) -> Message {
        for idx in 0..INTEGRITY_BITS_LENGTH {
            if bytes[idx] != INTEGRITY_BITS[idx] {
                return Message {
                    sender: String::from("Unknown"),
                    sender_length: String::from("Unknown").len() as u8,
                    content: MessageContent::Text("Encrypted Message".red().to_string()),
                };
            }
        }
        let sender_length: u8 = bytes[INTEGRITY_BITS_LENGTH];
        let sender = String::from_utf8(
            bytes[INTEGRITY_BITS_LENGTH + 1..INTEGRITY_BITS_LENGTH + sender_length as usize + 1]
                .to_vec(),
        )
        .unwrap();
        let variant_id = bytes[sender_length as usize + 1 + INTEGRITY_BITS_LENGTH];
        match variant_id {
            0 => {
                let message_content_text = String::from_utf8(
                    bytes[sender_length as usize + 2 + INTEGRITY_BITS_LENGTH..].to_vec(),
                )
                .unwrap();
                Message {
                    sender,
                    sender_length,
                    content: MessageContent::Text(message_content_text),
                }
            }
            1 => {
                let name_length = bytes[sender_length as usize + 2 + INTEGRITY_BITS_LENGTH];
                let name = String::from_utf8(
                    bytes[sender_length as usize + 3 + INTEGRITY_BITS_LENGTH
                        ..sender_length as usize
                            + 3
                            + name_length as usize
                            + INTEGRITY_BITS_LENGTH]
                        .to_vec(),
                )
                .unwrap();
                let file = bytes
                    [sender_length as usize + 3 + name_length as usize + INTEGRITY_BITS_LENGTH..]
                    .to_vec();
                Message {
                    sender,
                    sender_length,
                    content: MessageContent::File(name_length, name, file),
                }
            }
            _ => {
                panic!("Unknown message type");
            }
        }
    }
}
