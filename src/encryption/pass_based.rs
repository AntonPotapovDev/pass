use tindercrypt::cryptors::RingCryptor;

pub fn encrypt(orig_data: &Vec<u8>, pass: &str) -> Result<Vec<u8>, ()> {
    let cryptor = RingCryptor::new();
    match cryptor.seal_with_passphrase(pass.as_bytes(), orig_data) {
        Ok(encrypted) => Ok(encrypted),
        Err(_) => Err(()),
    }
}

pub fn decrypt(encrypted: &Vec<u8>, pass: &str) -> Result<Vec<u8>, ()> {
    let cryptor = RingCryptor::new();
    match cryptor.open(pass.as_bytes(), encrypted) {
        Ok(decrypted) => Ok(decrypted),
        Err(_) => Err(()),
    }
}
