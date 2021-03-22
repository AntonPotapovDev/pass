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
        if check(&args, 2) {
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
        if check(&args, 2) {
            let key = std::mem::replace(&mut args[0], String::new());
            let pass = std::mem::replace(&mut args[1], String::new());
            Ok(Box::new(super::Update{ key, pass }))
        } else { Err(()) }
    }

    fn cmd_usage(&self) -> String {
        String::from("<key> <new_password>")
    }
}

pub struct ExportBuilder;
impl CmdBuilder for ExportBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        if check(&args, 2) {
            let dest = std::mem::replace(&mut args[0], String::new());
            let key_dest = std::mem::replace(&mut args[1], String::new());
            Ok(Box::new(super::Export{ dest, key_dest }))
        } else { Err(()) }
    }

    fn cmd_usage(&self) -> String {
        String::from("<export_path> <key_path>")
    }
}

pub struct ImportBuilder;
impl CmdBuilder for ImportBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        if check(&args, 2) {
            let src = std::mem::replace(&mut args[0], String::new());
            let key_src = std::mem::replace(&mut args[1], String::new());
            Ok(Box::new(super::Import{ src, key_src }))
        } else { Err(()) }
    }

    fn cmd_usage(&self) -> String {
        String::from("<import_path> <key_path>")
    }
}

pub struct RenameBuilder;
impl CmdBuilder for RenameBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        if check(&args, 2) {
            let old = std::mem::replace(&mut args[0], String::new());
            let new = std::mem::replace(&mut args[1], String::new());
            Ok(Box::new(super::Rename{ old, new }))
        } else { Err(()) }
    }

    fn cmd_usage(&self) -> String {
        String::from("<old_key> <new_key>")
    }
}

pub struct ClearBuilder;
impl CmdBuilder for ClearBuilder {
    fn build(&self, mut _args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        Ok(Box::new(super::Clear))
    }

    fn cmd_usage(&self) -> String {
        String::new()
    }
}

pub struct CopyBuilder;
impl CmdBuilder for CopyBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        if check(&args, 1) {
            let key = std::mem::replace(&mut args[0], String::new());
            Ok(Box::new(super::Copy{ key }))
        } else { Err(()) }
    }

    fn cmd_usage(&self) -> String {
        String::from("<key_to_copy>")
    }
}

fn check(args: &Vec<String>, expected: usize) -> bool {
    args.len() >= expected
}
