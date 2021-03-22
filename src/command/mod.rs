pub mod builders;
pub mod resolver;

use clipboard::{ClipboardContext, ClipboardProvider};

use crate::context::Context;
use crate::impexp;

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
        match context.model.get(&self.key) {
            Some(pass) => println!("{}", pass),
            None => msg::no_such_key(),
        }
    }
}

impl From<String> for Show {
    fn from(key: String) -> Show {
        Show { key }
    }
}

pub struct Add {
    pub key: String,
    pub pass: String,
}

impl Command for Add {
    fn execute(&self, context: &mut Context) {
        match context.model.contains_key(&self.key) {
            true => msg::already_exist(),
            false => { context.model.insert(self.key.clone(), self.pass.clone()); },
        }
    }
}

impl From<(String, String)> for Add {
    fn from((key, pass): (String, String)) -> Add {
        Add { key, pass }
    }
}

pub struct Remove {
    pub key: String,
}

impl Command for Remove {
    fn execute(&self, context: &mut Context) {
        if let None = context.model.remove(&self.key) {
            msg::no_such_key()
        }
    }
}

impl From<String> for Remove {
    fn from(key: String) -> Remove {
        Remove { key }
    }
}

pub struct Update {
    pub key: String,
    pub pass: String,
}

impl Command for Update {
    fn execute(&self, context: &mut Context) {
        match context.model.contains_key(&self.key) {
            true => { context.model.insert(self.key.clone(), self.pass.clone()); },
            false => msg::no_such_key(),
        }
    }
}

impl From<(String, String)> for Update {
    fn from((key, pass): (String, String)) -> Update {
        Update { key, pass }
    }
}

pub struct Export {
    pub dest: String,
    pub key_dest: String,
}

impl Command for Export {
    fn execute(&self, context: &mut Context) {
        if let Err(err) = impexp::export(&context.data_file_path, &self.dest, &self.key_dest) {
            msg::export_error(err);
        }
    }
}

impl From<(String, String)> for Export {
    fn from((dest, key_dest): (String, String)) -> Export {
        Export { dest, key_dest }
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
                msg::import_error(err);
                return;
            },
        };

        let collisions = imorted_model.iter()
            .filter(|(key, _value)| context.model.contains_key(*key))
            .map(|(key, _value)| key.clone())
            .collect::<Vec<String>>();

        match collisions.len() > 0 {
            true => {
                msg::collision_detected();
                collisions.iter().for_each(|c| println!("{}", c));
            },
            false => imorted_model.into_iter().for_each(|(key, value)| { context.model.insert(key, value); }),
        }
    }
}

impl From<(String, String)> for Import {
    fn from((src, key_src): (String, String)) -> Import {
        Import { src, key_src }
    }
}

pub struct Rename {
    pub old: String,
    pub new: String,
}

impl Command for Rename {
    fn execute(&self, context: &mut Context) {
        match context.model.remove(&self.old) {
            Some(value) => { context.model.insert(self.new.clone(), value); },
            None => msg::no_such_key(),
        }
    }
}

impl From<(String, String)> for Rename {
    fn from((old, new): (String, String)) -> Rename {
        Rename { old, new }
    }
}

pub struct Clear;

impl Command for Clear {
    fn execute(&self, context: &mut Context) {
        context.model.clear();
    }
}

pub struct Copy {
    pub key: String,
}

impl Command for Copy {
    fn execute(&self, context: &mut Context) {
        match context.model.get(&self.key) {
            Some(value) => {
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                ctx.set_contents(value.clone()).unwrap();
            },
            None => msg::no_such_key(),
        }
    }
}

impl From<String> for Copy {
    fn from(key: String) -> Copy {
        Copy { key }
    }
}

mod msg {
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
}
