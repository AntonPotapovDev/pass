use crate::context::Context;
use super::{Command, msg};

use clipboard::{ClipboardContext, ClipboardProvider};

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
