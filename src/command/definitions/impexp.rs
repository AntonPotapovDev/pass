use crate::context::{self, Context};

use super::{
    Command,
    tools::{msg, merger, encryption, dialog},
};

use std::fs::File;
use std::io::{Read, Write};

pub struct Export {
    pub dest: String,
    pub clear: bool,
}

const DEFAULT_IMPORT_EXPORT_FILENAME: &str = "data_exported";

impl Command for Export {
    fn execute(self: Box<Self>, context: &mut Context) {
        let mut file = File::open(&context.data_file_path).unwrap();

        let mut data = vec![];
        file.read_to_end(&mut data).unwrap();

        let pass = match dialog::ask_for_password(true) {
            Ok(p) => p,
            Err(err) => {
                msg::pass_read_error(err);
                return;
            }
        };

        let result = match encryption::encrypt(&data, &pass) {
            Ok(d) => d,
            Err(_) => {
                msg::encryption_failed();
                return;
            },
        };

        let dest_path = if self.dest.len() > 0 { self.dest } else { make_default_path() };

        match File::create(&dest_path) {
            Ok(mut f) => {
                f.write(&result).unwrap();
                if self.clear { context.model.clear() }
            },
            Err(_) => {
                msg::failed_writing(&dest_path);
                return;
            }
        }
    }
}

impl From::<(String, bool)> for Export {
    fn from((dest, clear): (String, bool)) -> Export {
        Export { dest, clear }
    }
}

pub struct Import {
    pub src: String,
    pub clear: bool,
}

impl Command for Import {
    fn execute(self: Box<Self>, context: &mut Context) {
        let src_path = if self.src.len() > 0 { self.src } else { make_default_path() };

        let data = match read_file(&src_path) {
            Ok(d) => d,
            Err(_) => {
                msg::failed_reading(&src_path);
                return;
            },
        };

        let pass = match dialog::ask_for_password(false) {
            Ok(p) => p,
            Err(err) => {
                msg::pass_read_error(err);
                return;
            }
        };

        let str_model = match encryption::decrypt(&data, &pass) {
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

        let collisions = context::find_collisions(&imported_model, &mut context.model);

        match collisions.len() > 0 {
            true => {
                msg::collision_detected();
                collisions.iter().for_each(|c| println!("{}", c));
                merger::interactive_merge(imported_model, &mut context.model);
            },
            false => context::merge_models(imported_model, &mut context.model),
        }
    }
}

impl From::<(String, bool)> for Import {
    fn from((src, clear): (String, bool)) -> Import {
        Import { src, clear }
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

fn make_default_path() -> String {
    let mut dir = std::env::current_exe().unwrap();
    dir.pop();
    dir.push(DEFAULT_IMPORT_EXPORT_FILENAME);
    String::from(dir.to_str().unwrap())
}
