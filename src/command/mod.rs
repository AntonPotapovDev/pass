pub mod builders;

use crate::context::Context;
use crate::impexp::{self, export::ExportErr, import::ImportError};

pub trait Command {
    fn execute(&self, model: &mut Context);
}

pub struct List;

impl Command for List {
    fn execute(&self, context: &mut Context) {
        context.model.iter().for_each(|(key, _value)| println!("{}", key));
    }
}

pub struct Show {
    pub key: String,
}

impl Command for Show {
    fn execute(&self, context: &mut Context) {
        if let Some(pass) = context.model.get(&self.key) {
            println!("{}", pass);
        } else {
            println!("No passwords for that key");
        }
    }
}

pub struct Add {
    pub key: String,
    pub pass: String,
}

impl Command for Add {
    fn execute(&self, context: &mut Context) {
        if context.model.contains_key(&self.key) {
            println!("Password for the given key is already exist");
        } else {
            context.model.insert(self.key.clone(), self.pass.clone());
        }
    }
}

pub struct Remove {
    pub key: String,
}

impl Command for Remove {
    fn execute(&self, context: &mut Context) {
        if context.model.contains_key(&self.key) {
            context.model.remove(&self.key);
        } else {
            println!("No passwords for that key");
        }
    }
}

pub struct Update {
    pub key: String,
    pub pass: String,
}

impl Command for Update {
    fn execute(&self, context: &mut Context) {
        if context.model.contains_key(&self.key) {
            context.model.insert(self.key.clone(), self.pass.clone());
        } else {
            println!("No passwords for that key");
        }
    }
}

pub struct Export {
    pub dest: String,
    pub key_dest: String,
}

impl Command for Export {
    fn execute(&self, context: &mut Context) {
        if let Err(err) = impexp::export(&context.data_file_path, &self.dest, &self.key_dest) {
            match err {
                ExportErr::EncryptionError => println!("Failed to encrypt data"),
                ExportErr::FSError => println!("Failed to write export data"),
                ExportErr::KeyGenError => println!("Failed to generate encryption keys"),
            }
        }
    }
}

pub struct Import {
    pub src: String,
    pub key_src: String,
}

impl Command for Import {
    fn execute(&self, context: &mut Context) {
        let imorted_model = match impexp::import(&self.src, &self.key_src) {
            Ok(m) => m,
            Err(err) => {
                match err {
                    ImportError::DeryptionError => println!("Failed to decrypt file"),
                    ImportError::FSError => println!("Failed to read import data"),
                    ImportError::KeyGenError => println!("Failed to build encryption key"),
                    ImportError::InvalidFile => println!("Invalid import file"),
                }
                return;
            },
        };

        let collisions = imorted_model.iter()
            .filter(|(key, _value)| context.model.contains_key(*key))
            .map(|(key, _value)| key.clone())
            .collect::<Vec<String>>();

        if collisions.len() > 0 {
            println!("Key collisions detected, command aborted");
            println!("Resolve collisions for the following keys:");
            collisions.iter().for_each(|c| println!("{}", c));
        } else {
            imorted_model.into_iter().for_each(|(key, value)| { context.model.insert(key, value); });
        }
    }
}
