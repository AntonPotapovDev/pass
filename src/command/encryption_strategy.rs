use std::fs::File;
use std::io::{Read, Write};

use crate::encryption;
use super::msg;

pub trait EncryptionStrategy {
    fn encrypt(&self, data: &Vec<u8>) -> Result<Vec<u8>, ()>;
    fn decrypt(&self, data: &Vec<u8>) -> Result<Vec<u8>, ()>;
}

pub struct KeyBased {
    key_path: String,
}

impl From<String> for KeyBased {
    fn from(key_path: String) -> KeyBased {
        KeyBased { key_path }
    }
}

impl EncryptionStrategy for KeyBased {
    fn encrypt(&self, data: &Vec<u8>) -> Result<Vec<u8>, ()> {
        let result = match encryption::key_based::encrypt(&data) {
            Ok(r) => r,
            Err(_) => return Err(()),
        };

        match File::create(&self.key_path) {
            Ok(mut f) => {
                f.write(&result.key).unwrap();
                Ok(result.data)
            },
            Err(_) => {
                msg::failed_writing(&self.key_path);
                Err(())
            },
        }
    }

    fn decrypt(&self, data: &Vec<u8>) -> Result<Vec<u8>, ()> {
        let key = match File::open(&self.key_path) {
            Ok(mut f) => {
                let mut key_data = vec![];
                f.read_to_end(&mut key_data).unwrap();
                key_data
            },
            Err(_) => {
                msg::failed_reading(&self.key_path);
                return Err(())
            },
        };

        encryption::key_based::decrypt(data, &key)
    }
}

pub struct PassBased {
    pass: String,
}

impl From<String> for PassBased {
    fn from(pass: String) -> PassBased {
        PassBased { pass }
    }
}

impl EncryptionStrategy for PassBased {
    fn encrypt(&self, data: &Vec<u8>) -> Result<Vec<u8>, ()> {
        encryption::pass_based::encrypt(data, &self.pass)
    }

    fn decrypt(&self, data: &Vec<u8>) -> Result<Vec<u8>, ()> {
        encryption::pass_based::decrypt(data, &self.pass)
    }
}
