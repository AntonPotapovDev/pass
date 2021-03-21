use super::Command;

pub trait CmdBuilder {
    fn build(&self, args: Vec<String>) -> Result<Box<dyn Command>, ()>;
    fn cmd_usage(&self) -> String;
}

pub struct ListBuilder;
impl CmdBuilder for ListBuilder {
    fn build(&self, _args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        Ok(Box::new(super::List))
    }

    fn cmd_usage(&self) -> String {
        String::new()
    }
}

pub struct ShowBuilder;
impl CmdBuilder for ShowBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        if check(&args, 1) {
            let key = std::mem::replace(&mut args[0], String::new());
            Ok(Box::new(super::Show{ key }))
        } else { Err(()) }
    }

    fn cmd_usage(&self) -> String {
        String::from("<key>")
    }
}

pub struct AddBuilder;
impl CmdBuilder for AddBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        if check(&args, 1) {
            let key = std::mem::replace(&mut args[0], String::new());
            let pass = std::mem::replace(&mut args[1], String::new());
            Ok(Box::new(super::Add{ key, pass }))
        } else { Err(()) }
    }

    fn cmd_usage(&self) -> String {
        String::from("<key> <password>")
    }
}

pub struct RemoveBuilder;
impl CmdBuilder for RemoveBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        if check(&args, 1) {
            let key = std::mem::replace(&mut args[0], String::new());
            Ok(Box::new(super::Remove{ key }))
        } else { Err(()) }
    }

    fn cmd_usage(&self) -> String {
        String::from("<key_to_delete>")
    }
}

pub struct UpdateBuilder;
impl CmdBuilder for UpdateBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        if check(&args, 1) {
            let key = std::mem::replace(&mut args[0], String::new());
            let pass = std::mem::replace(&mut args[1], String::new());
            Ok(Box::new(super::Update{ key, pass }))
        } else { Err(()) }
    }

    fn cmd_usage(&self) -> String {
        String::from("<key> <new_password>")
    }
}

fn check(args: &Vec<String>, expected: usize) -> bool {
    args.len() < expected
}
