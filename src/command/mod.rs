pub mod builders;

use std::env;

use crate::model::PassListModel;
use crate::impexp::{self, export::ExportErr, import::ImportError};

pub trait Command {
    fn execute(&self, model: &mut PassListModel);
}

pub struct List;

impl Command for List {
    fn execute(&self, model: &mut PassListModel) {
        model.iter().for_each(|(key, _value)| println!("{}", key));
    }
}

pub struct Show {
    pub key: String,
}

impl Command for Show {
    fn execute(&self, model: &mut PassListModel) {
        if let Some(pass) = model.get(&self.key) {
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
    fn execute(&self, model: &mut PassListModel) {
        if model.contains_key(&self.key) {
            println!("Password for the given key is already exist");
        } else {
            model.insert(self.key.clone(), self.pass.clone());
        }
    }
}

pub struct Remove {
    pub key: String,
}

impl Command for Remove {
    fn execute(&self, model: &mut PassListModel) {
        if model.contains_key(&self.key) {
            model.remove(&self.key);
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
    fn execute(&self, model: &mut PassListModel) {
        if model.contains_key(&self.key) {
            model.insert(self.key.clone(), self.pass.clone());
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
    fn execute(&self, _model: &mut PassListModel) {
        // TODO: make possible to bring model file path to this context
        let mut dir = env::current_exe().unwrap();
        dir.pop();
        dir.push(".data");

        if let Err(err) = impexp::export(dir.to_str().unwrap(), &self.dest, &self.key_dest) {
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
    fn execute(&self, model: &mut PassListModel) {
        let imorted_model = match impexp::import(&self.src, &self.key_src) {
            Ok(m) => m,
            Err(err) => {
                match err {
                    ImportError::DeryptionError => println!("Failed to encrypt file"),
                    ImportError::FSError => println!("Failed to read import data"),
                    ImportError::KeyGenError => println!("Failed to build encryption key"),
                }
                return;
            },
        };

        let collisions = imorted_model.iter()
            .filter(|(key, _value)| model.contains_key(*key))
            .map(|(key, _value)| key.clone())
            .collect::<Vec<String>>();

        if collisions.len() > 0 {
            println!("Key collisions detected, command aborted");
            println!("Resolve collisions for the following keys:");
            collisions.iter().for_each(|c| println!("{}", c));
        } else {
            imorted_model.into_iter().for_each(|(key, value)| { model.insert(key, value); });
        }
    }
}
