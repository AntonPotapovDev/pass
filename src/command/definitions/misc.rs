use crate::context::{self, Context, PassListModel};

use super::{
    Command,
    tools::{msg, dialog, merger},
};

pub struct MultiAdd {
    pub keys: Vec<String>,
}

impl Command for MultiAdd {
    fn execute(&self, context: &mut Context) {
        let pass = match dialog::ask_for_password(true) {
            Ok(p) => p,
            Err(err) => {
                msg::pass_read_error(err);
                return;
            },
        };

        let mut extension = PassListModel::new();

        self.keys.iter().for_each(|k| { extension.insert(k.clone(), pass.clone()); });

        if let Err(collisions) = context::safe_merge(&extension, &mut context.model) {
            msg::collision_detected();
            collisions.iter().for_each(|c| println!("{}", c));
            merger::interactive_merge(extension, &mut context.model);
        }
    }
}

impl From::<Vec<String>> for MultiAdd {
    fn from(keys: Vec<String>) -> MultiAdd {
        MultiAdd { keys }
    }
}

pub struct MultiRemove {
    pub keys: Vec<String>,
}

impl Command for MultiRemove {
    fn execute(&self, context: &mut Context) {
        match dialog::confirm(msg::strings::MRM) {
            Ok(true) => self.keys.iter().for_each(|key| {
                if !context.model.contains_key(key) {
                    msg::no_such_key_warning(key);
                    return;
                }
                context.model.remove(key);
            }),
            Err(_) => msg::input_failed(),
            _ => (),
        }
    }
}

pub struct MultiUpdate {
    pub keys: Vec<String>,
}

impl Command for MultiUpdate {
    fn execute(&self, context: &mut Context) {
        let pass = match dialog::ask_for_password(true) {
            Ok(p) => p,
            Err(err) => {
                msg::pass_read_error(err);
                return;
            },
        };

        for key in &self.keys {
            if !context.model.contains_key(key) {
                msg::no_such_key_warning(key);
                continue;
            }

            context.model.insert(key.clone(), pass.clone());
        }
    }
}

impl From::<Vec<String>> for MultiUpdate {
    fn from(keys: Vec<String>) -> MultiUpdate {
        MultiUpdate { keys }
    }
}
