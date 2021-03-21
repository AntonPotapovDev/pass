use rsa::{PublicKey, RSAPrivateKey, RSAPublicKey, PaddingScheme};
use rand::rngs::OsRng;
use rsa_export::Encode;

use std::fs::File;
use std::io::{Read, Write};

pub enum ExportErr {
    FSError,
    KeyGenError,
    EncryptionError,
}

pub fn export(src: &str, dst: &str, key: &str) -> Result<(), ExportErr> {
    let mut file = File::open(src).unwrap();

    let mut data = vec![];
    file.read_to_end(&mut data).unwrap();

    // create keys
    let mut rng = OsRng;
    let bits = 2048;

    let private_key = match RSAPrivateKey::new(&mut rng, bits) {
        Ok(key) => key,
        Err(_) => return Err(ExportErr::KeyGenError),
    };

    let public_key = RSAPublicKey::from(&private_key);

    // encrypt
    let padding = PaddingScheme::new_pkcs1v15_encrypt();

    let export_data = match public_key.encrypt(&mut rng, padding, &data) {
        Ok(ed) => ed,
        Err(_) => return Err(ExportErr::EncryptionError),
    };

    // export data
    let mut file = match File::create(dst) {
        Ok(f) => f,
        Err(_) => return Err(ExportErr::FSError),
    };

    file.write(&export_data[..]).unwrap();

    // export key
    let encoded = private_key.as_pkcs1().unwrap();

    let mut file = match File::create(key) {
        Ok(f) => f,
        Err(_) => return Err(ExportErr::FSError),
    };

    file.write(&encoded[..]).unwrap();

    Ok(())
}
