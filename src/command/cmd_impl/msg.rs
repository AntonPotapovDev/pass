use crate::impexp::{import::ImportError, export::ExportError};

pub fn no_such_key() {
    println!("No passwords for that key");
}

pub fn already_exist() {
    println!("Password for the given key is already exist");
}

pub fn collision_detected() {
    println!("Key collisions detected, command aborted");
    println!("Resolve collisions for the following keys:");
}

pub fn export_error(err: ExportError) {
    match err {
        ExportError::EncryptionError => println!("Failed to encrypt data"),
        ExportError::FSError => println!("Failed to write export data"),
        ExportError::KeyGenError => println!("Failed to generate encryption keys"),
    }
}

pub fn import_error(err: ImportError) {
    match err {
        ImportError::DeryptionError => println!("Failed to decrypt file"),
        ImportError::FSError => println!("Failed to read import data"),
        ImportError::KeyGenError => println!("Failed to build encryption key"),
        ImportError::InvalidFile => println!("Invalid import file"),
    }
}