use std::io;
use std::io::prelude::*; 

pub fn confirm(message: &str) -> Result<bool, io::Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    loop {
        print!("{} (Y/N): ", message);
        io::stdout().flush().unwrap();

        match stdin.read_line(&mut buffer) {
            Ok(_) => match &buffer.trim().to_lowercase()[..] {
                "yes" | "y" => return Ok(true),
                "no" | "n" => return Ok(false),
                _ => (),
            },
            Err(err) => return Err(err),
        }
    }
}

pub enum PassReadError {
    SystemError,
    ConfirmationError,
}

pub fn ask_for_password(confirm: bool) -> Result<String, PassReadError> {
    print!("Password: ");
    io::stdout().flush().unwrap();
    let pass = match rpassword::read_password() {
        Ok(p) => p,
        Err(_) => return Err(PassReadError::SystemError),
    };

    if !confirm { return Ok(pass) }

    print!("Confirm: ");
    io::stdout().flush().unwrap();
    match rpassword::read_password()  {
        Ok(p) => if p == pass { Ok(pass) } else { Err(PassReadError::ConfirmationError) },
        Err(_) => Err(PassReadError::SystemError),
    } 
}
