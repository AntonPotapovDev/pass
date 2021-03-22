use crate::context::{self, Context};
use super::{Command, msg};
use crate::impexp;

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

        if let Err(collisions) = context::merge_models(imorted_model, &mut context.model) {
            msg::collision_detected();
            collisions.iter().for_each(|c| println!("{}", c));
        }
    }
}

impl From<(String, String)> for Import {
    fn from((src, key_src): (String, String)) -> Import {
        Import { src, key_src }
    }
}
