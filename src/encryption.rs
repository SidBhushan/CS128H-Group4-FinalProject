pub struct Encryptor {
    pub key: Option<Vec<u8>>,
}

impl Encryptor {
    pub fn new() -> Encryptor {
        Encryptor { key: None }
    }

    pub fn apply_encryption(&self, data: &mut Vec<u8>) {
        if let Some(key) = &self.key {
            Encryptor::xor(data, key);
        }
    }

    fn xor(data: &mut Vec<u8>, key: &Vec<u8>) {
        let key_length = key.len();
        for (idx, byte) in data.iter_mut().enumerate() {
            *byte = *byte ^ key.get(idx % key_length).unwrap();
        }
    }
}
