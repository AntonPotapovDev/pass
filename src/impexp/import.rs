use rsa::{RSAPrivateKey, PaddingScheme};

use crate::model::{self, PassListModel};

use std::fs::File;
use std::io::Read;

pub enum ImportError {
    FSError,
    KeyGenError,
    DeryptionError,
}

pub fn import(data_path: &str, key_path: &str) -> Result<PassListModel, ImportError> {
    // read encrypted data
    let data = match read_file(data_path) {
        Ok(d) => d,
        Err(_) => return Err(ImportError::FSError),
    };

    // read key
    let key = match read_file(key_path) {
        Ok(d) => d,
        Err(_) => return Err(ImportError::FSError),
    };

    // create key
    let private_key = match RSAPrivateKey::from_pkcs1(&key[..]) {
        Ok(k) => k,
        Err(_) => return Err(ImportError::KeyGenError),
    };

    // decrypt
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let dec_data = match private_key.decrypt(padding, &data) {
        Ok(d) => d,
        Err(_) => return Err(ImportError::DeryptionError),
    };

    let lines = String::from_utf8(dec_data)
        .unwrap()
        .split("\n")
        .map(|x| String::from(x))
        .filter(|x| !x.is_empty())
        .collect::<Vec<String>>();

    let mut model = PassListModel::new();

    lines.iter().for_each(|l| {
        let (key, value) = model::pair_from_line(l);
        model.insert(key, value);
    });

    Ok(model)
}

fn read_file(path: &str) -> Result<Vec<u8>, ()> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Err(()),
    };

    let mut data = vec![];
    match file.read_to_end(&mut data) {
        Ok(_) => Ok(data),
        Err(_) => Err(()),
    }
}
