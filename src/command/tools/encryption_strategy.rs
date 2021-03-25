use std::fs::File;
use std::io::{Read, Write};

use super::{dialog, msg, encryption};

pub enum ESError {
    InnerError,
    EncryptionError,
}

pub trait EncryptionStrategy {
    fn encrypt(&self, data: &Vec<u8>) -> Result<Vec<u8>, ESError>;
    fn decrypt(&self, data: &Vec<u8>) -> Result<Vec<u8>, ESError>;
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
    fn encrypt(&self, data: &Vec<u8>) -> Result<Vec<u8>, ESError> {
        let result = match encryption::key_based::encrypt(&data) {
            Ok(r) => r,
            Err(_) => return Err(ESError::EncryptionError),
        };

        match File::create(&self.key_path) {
            Ok(mut f) => {
                f.write(&result.key).unwrap();
                Ok(result.data)
            },
            Err(_) => {
                msg::failed_writing(&self.key_path);
                Err(ESError::InnerError)
            },
        }
    }

    fn decrypt(&self, data: &Vec<u8>) -> Result<Vec<u8>, ESError> {
        let key = match File::open(&self.key_path) {
            Ok(mut f) => {
                let mut key_data = vec![];
                f.read_to_end(&mut key_data).unwrap();
                key_data
            },
            Err(_) => {
                msg::failed_reading(&self.key_path);
                return Err(ESError::InnerError);
            },
        };

        match encryption::key_based::decrypt(data, &key) {
            Ok(res) => Ok(res),
            Err(_) => Err(ESError::EncryptionError),
        }
    }
}

pub struct PassBased;

impl EncryptionStrategy for PassBased {
    fn encrypt(&self, data: &Vec<u8>) -> Result<Vec<u8>, ESError> {
        match dialog::ask_for_password(true) {
            Ok(pass) => match encryption::pass_based::encrypt(data, &pass) {
                Ok(res) => Ok(res),
                Err(_) => Err(ESError::EncryptionError),
            },
            Err(err) => {
                msg::pass_read_error(err);
                Err(ESError::InnerError)
            },
        }
    }

    fn decrypt(&self, data: &Vec<u8>) -> Result<Vec<u8>, ESError> {
        match dialog::ask_for_password(false) {
            Ok(pass) => match encryption::pass_based::decrypt(data, &pass) {
                Ok(res) => Ok(res),
                Err(_) => Err(ESError::EncryptionError),
            },
            Err(err) => {
                msg::pass_read_error(err);
                Err(ESError::InnerError)
            },
        }
    }
}
