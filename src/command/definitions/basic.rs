use crate::context::Context;

use super::{
    Command,
    tools::{msg, dialog},
};

use clipboard::{ClipboardContext, ClipboardProvider};

pub struct List;

impl Command for List {
    fn execute(self: Box<Self>, context: &mut Context) {
        let mut list = context.model.iter()
            .map(|(key, _value)| key)
            .collect::<Vec<&String>>();

        list.sort();
        
        list.iter().for_each(|e| println!("{}", e));
    }
}

pub struct Show {
    pub key: String,
}

impl Command for Show {
    fn execute(self: Box<Self>, context: &mut Context) {
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
}

impl Command for Add {
    fn execute(self: Box<Self>, context: &mut Context) {
        match context.model.contains_key(&self.key) {
            true => msg::already_exist(),
            false => match dialog::ask_for_password(true) {
                Ok(pass) => { context.model.insert(self.key, pass); },
                Err(err) => msg::pass_read_error(err),
            }
        }
    }
}

impl From<String> for Add {
    fn from(key: String) -> Add {
        Add { key }
    }
}

pub struct Remove {
    pub key: String,
}

impl Command for Remove {
    fn execute(self: Box<Self>, context: &mut Context) {
        match context.model.contains_key(&self.key) {
            true => match dialog::confirm(msg::strings::RM) {
                Ok(true) => { context.model.remove(&self.key); },
                Err(_) => msg::input_failed(),
                _ => (),
            },
            false => msg::no_such_key(),
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
}

impl Command for Update {
    fn execute(self: Box<Self>, context: &mut Context) {
        match context.model.contains_key(&self.key) {
            true => match dialog::ask_for_password(true) {
                Ok(pass) => { context.model.insert(self.key, pass); },
                Err(err) => msg::pass_read_error(err),
            },
            false => msg::no_such_key(),
        }
    }
}

impl From<String> for Update {
    fn from(key: String) -> Update {
        Update { key }
    }
}

pub struct Rename {
    pub old: String,
    pub new: String,
}

impl Command for Rename {
    fn execute(self: Box<Self>, context: &mut Context) {
        if !context.model.contains_key(&self.old) {
            msg::no_such_key();
            return;
        }

        if context.model.contains_key(&self.new) {
            msg::failed_renaming();
            return;
        }

        let value = context.model.remove(&self.old).unwrap();
        context.model.insert(self.new, value);
    }
}

impl From<(String, String)> for Rename {
    fn from((old, new): (String, String)) -> Rename {
        Rename { old, new }
    }
}

pub struct Clear;

impl Command for Clear {
    fn execute(self: Box<Self>, context: &mut Context) {
        match dialog::confirm(msg::strings::CLEAR) {
            Ok(answer) => if answer { context.model.clear(); },
            Err(_) => msg::input_failed(),
        }
    }
}

pub struct Copy {
    pub key: String,
}

impl Command for Copy {
    fn execute(self: Box<Self>, context: &mut Context) {
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

pub struct Paste {
    pub key: String,
}

impl Command for Paste {
    fn execute(self: Box<Self>, context: &mut Context) {
        match context.model.contains_key(&self.key) {
            true => msg::already_exist(),
            false => {
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                match ctx.get_contents() {
                    Ok(pass) => { context.model.insert(self.key, pass); },
                    Err(_) => msg::pass_read_error(dialog::PassReadError::SystemError),
                }
            }
        }
    }
}

impl From<String> for Paste {
    fn from(key: String) -> Paste {
        Paste { key }
    }
}
