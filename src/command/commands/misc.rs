use crate::context::{self, Context, PassListModel};
use super::{Command, msg};

pub struct MultiAdd {
    pub keys: Vec<String>,
    pub pass: String,
}

impl Command for MultiAdd {
    fn execute(&self, context: &mut Context) {
        let mut extension = PassListModel::new();
        self.keys.iter().for_each(|k| { extension.insert(k.clone(), self.pass.clone()); });

        if let Err(collisions) = context::merge_models(extension, &mut context.model) {
            msg::collision_detected();
            collisions.iter().for_each(|c| println!("{}", c));
        }
    }
}

impl From::<(Vec<String>, String)> for MultiAdd {
    fn from((keys, pass): (Vec<String>, String)) -> MultiAdd {
        MultiAdd { keys, pass }
    }
}

pub struct MultiRemove {
    pub keys: Vec<String>,
}

impl Command for MultiRemove {
    fn execute(&self, context: &mut Context) {
        self.keys.iter().for_each(|key| { context.model.remove(key); });
    }
}

pub struct MultiUpdate {
    pub keys: Vec<String>,
    pub pass: String,
}

impl Command for MultiUpdate {
    fn execute(&self, context: &mut Context) {
        for key in &self.keys {
            if !context.model.contains_key(key) { continue; }
            context.model.insert(key.clone(), self.pass.clone());
        }
    }
}

impl From::<(Vec<String>, String)> for MultiUpdate {
    fn from((keys, pass): (Vec<String>, String)) -> MultiUpdate {
        MultiUpdate { keys, pass }
    }
}
