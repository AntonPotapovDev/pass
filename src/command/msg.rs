use super::dialog::PassReadError;

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

pub fn encryption_failed() {
    println!("Failed to encrypt data");
}

pub fn failed_writing(filename: &str) {
    println!("Could not write to \"{}\"", filename);
}

pub fn decryption_failed() {
    println!("Failed to decrypt file");
}

pub fn failed_reading(filename: &str) {
    println!("Could not read from \"{}\"", filename);
}

pub fn bad_file() {
    println!("Invalid import file");
}

pub fn input_failed() {
    println!("Could not read the input");
}

pub fn pass_read_error(err: PassReadError) {
    match err {
        PassReadError::SystemError => input_failed(),
        PassReadError::ConfirmationError => println!("Password mismatch"),
    }
}

pub mod strings {
    pub const CLEAR: &str = "Are you sure you want to delete all your passwords?";
    pub const RM: &str = "Are you sure you want to delete this password?";
    pub const MRM: &str = "Are you sure you want to delete these passwords?";
}
