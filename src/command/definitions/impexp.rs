use crate::context::{self, Context};

use super::{
    Command,
    tools::{msg, merger, encryption_strategy::EncryptionStrategy},
};

use std::fs::File;
use std::io::{Read, Write};

pub struct Export {
    pub dest: String,
    pub encryption_strategy: Box<dyn EncryptionStrategy>,
    pub clear: bool,
}

impl Command for Export {
    fn execute(&self, context: &mut Context) {
        let mut file = File::open(&context.data_file_path).unwrap();

        let mut data = vec![];
        file.read_to_end(&mut data).unwrap();

        let result = match self.encryption_strategy.encrypt(&data) {
            Ok(d) => d,
            Err(_) => {
                msg::encryption_failed();
                return;
            },
        };

        match File::create(&self.dest) {
            Ok(mut f) => {
                f.write(&result).unwrap();
                if self.clear { context.model.clear() }
            },
            Err(_) => {
                msg::failed_writing(&self.dest);
                return;
            }
        }
    }
}

impl From::<(String, Box::<dyn EncryptionStrategy>, bool)> for Export {
    fn from((dest, encryption_strategy, clear): (String, Box::<dyn EncryptionStrategy>, bool)) -> Export {
        Export { dest, encryption_strategy, clear }
    }
}

pub struct Import {
    pub src: String,
    pub encryption_strategy: Box<dyn EncryptionStrategy>,
    pub clear: bool,
}

impl Command for Import {
    fn execute(&self, context: &mut Context) {
        let data = match read_file(&self.src) {
            Ok(d) => d,
            Err(_) => {
                msg::failed_reading(&self.src);
                return;
            },
        };

        let str_model = match self.encryption_strategy.decrypt(&data) {
            Ok(bytes) => match String::from_utf8(bytes) {
                Ok(s) => s,
                Err(_) => {
                    msg::bad_file();
                    return;
                },
            },
            Err(_) => {
                msg::decryption_failed();
                return;
            },
        };

        let imported_model = match context::model_from_string(str_model) {
            Ok(m) => m,
            Err(_) => {
                msg::bad_file();
                return;
            },
        };

        if self.clear {
            context.model = imported_model;
            return;
        }

        if let Err(collisions) = context::safe_merge(&imported_model, &mut context.model) {
            msg::collision_detected();
            collisions.iter().for_each(|c| println!("{}", c));
            merger::interactive_merge(imported_model, &mut context.model);
        }
    }
}

impl From::<(String, Box::<dyn EncryptionStrategy>, bool)> for Import {
    fn from((src, encryption_strategy, clear): (String, Box::<dyn EncryptionStrategy>, bool)) -> Import {
        Import { src, encryption_strategy, clear }
    }
}

fn read_file(path: &str) -> Result<Vec<u8>, ()> {
    match File::open(path) {
        Ok(mut f) => {
            let mut data = vec![];
            match f.read_to_end(&mut data) {
                Ok(_) => Ok(data),
                Err(_) => Err(()),
            }
        },
        Err(_) => return Err(()),
    }
}
