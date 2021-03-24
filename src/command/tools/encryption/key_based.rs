use rsa::{PublicKey, RSAPrivateKey, RSAPublicKey, PaddingScheme};
use rand::rngs::OsRng;
use rsa_export::Encode;

pub struct RSAResult {
    pub data: Vec<u8>,
    pub key: Vec<u8>,
}

pub fn encrypt(orig_data: &Vec<u8>) -> Result<RSAResult, ()> {
    // create keys
    let mut rng = OsRng;
    let bits = 2048;

    let private_key = match RSAPrivateKey::new(&mut rng, bits) {
        Ok(key) => key,
        Err(_) => return Err(()),
    };

    let public_key = RSAPublicKey::from(&private_key);

    // encrypt
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let data = match public_key.encrypt(&mut rng, padding, orig_data) {
        Ok(ed) => ed,
        Err(_) => return Err(()),
    };

    // export key
    let key = private_key.as_pkcs1().unwrap();

    Ok(RSAResult{ data, key })
}

pub fn decrypt(encrypted_data: &Vec<u8>, key: &Vec<u8>) -> Result<Vec<u8>, ()> {
    // create key
    let private_key = match RSAPrivateKey::from_pkcs1(&key) {
        Ok(k) => k,
        Err(_) => return Err(()),
    };

    // decrypt
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let data = match private_key.decrypt(padding, encrypted_data) {
        Ok(d) => d,
        Err(_) => return Err(()),
    };

    Ok(data)
}