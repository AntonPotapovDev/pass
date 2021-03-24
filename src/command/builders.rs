use super::{Command, encryption_strategy::{self, EncryptionStrategy}};

const CLEAR_FLAG: &str = "-c";

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
        build_from_one::<super::Show>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from("<key>")
    }
}

pub struct AddBuilder;
impl CmdBuilder for AddBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_from_one::<super::Add>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from("<key>")
    }
}

pub struct RemoveBuilder;
impl CmdBuilder for RemoveBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_from_one::<super::Remove>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from("<key_to_delete>")
    }
}

pub struct UpdateBuilder;
impl CmdBuilder for UpdateBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_from_one::<super::Update>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from("<key>")
    }
}

pub struct RSAExportBuilder;
impl CmdBuilder for RSAExportBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_impexp_key_based::<super::Export>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from("<export_path> <key_path> [-c] (c - for clear)")
    }
}

pub struct RSAImportBuilder;
impl CmdBuilder for RSAImportBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_impexp_key_based::<super::Import>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from("<import_path> <key_path> [-c] (c - for clear)")
    }
}

pub struct PassBasedExportBuilder;
impl CmdBuilder for PassBasedExportBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_impexp_pass_based::<super::Export>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from("<export_path> [-c] (c - for clear)")
    }
}

pub struct PassBasedImportBuilder;
impl CmdBuilder for PassBasedImportBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_impexp_pass_based::<super::Import>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from("<import_path> [-c] (c - for clear)")
    }
}

pub struct RenameBuilder;
impl CmdBuilder for RenameBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_from_two::<super::Rename>(&mut args)
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
        build_from_one::<super::Copy>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from("<key_to_copy>")
    }
}

pub struct MultiAddBuilder;
impl CmdBuilder for MultiAddBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_from_list::<super::MultiAdd>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from("<key> [, <key>, <key>, ... ]")
    }
}

pub struct MultiRemoveBuilder;
impl CmdBuilder for MultiRemoveBuilder {
    fn build(&self, args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        if args.len() < 1 { return Err(()); }
        Ok(Box::new(super::MultiRemove{ keys: args }))
    }

    fn cmd_usage(&self) -> String {
        String::from("<key> [, <key>, <key>, ... ]")
    }
}

pub struct MultiUpdateBuilder;
impl CmdBuilder for MultiUpdateBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_from_list::<super::MultiUpdate>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from("<key> [, <key>, <key>, ... ]")
    }
}

fn unpack_one(args: &mut Vec<String>, index: usize) -> String {
    std::mem::replace(&mut args[index], String::new())
}

fn unpack_two(args: &mut Vec<String>) -> (String, String) {
    let first = std::mem::replace(&mut args[0], String::new());
    let second = std::mem::replace(&mut args[1], String::new());
    (first, second)
}

fn build_from_one<T>(args: &mut Vec<String>) -> Result<Box<dyn Command>, ()>
    where T: 'static + From::<String> + Command {
    match args.len() >= 1 {
        true => Ok(Box::new(T::from(unpack_one(args, 0)))),
        false => Err(()),
    }
}

fn build_from_two<T>(args: &mut Vec<String>) -> Result<Box<dyn Command>, ()>
    where T: 'static + From::<(String, String)> + Command {
    match args.len() >= 2 {
        true => {
            let (first, second) = unpack_two(args);
            Ok(Box::new(T::from((first, second))))
        }
        false => Err(()),
    }
}

fn build_from_list<T>(args: &mut Vec<String>) -> Result<Box<dyn Command>, ()>
    where T: 'static + From::<Vec<String>> + Command {
    match args.len() < 1 {
        true => Err(()),
        false => Ok(Box::new(T::from(args.clone()))), 
    } 
}

fn build_impexp_key_based<C>(args: &mut Vec<String>) -> Result<Box<dyn Command>, ()>
    where C: 'static + From::<(String, Box::<dyn EncryptionStrategy>, bool)> + Command {
    match args.len() >= 2 {
        true => {
            let (data, key) = unpack_two(args);
            let clear = if args.len() > 2 { &unpack_one(args, 2) == CLEAR_FLAG } else { false };
            let strategy = Box::new(encryption_strategy::KeyBased::from(key));
            let cmd = Box::new(C::from((data, strategy, clear)));
            Ok(cmd)
        },
        false => Err(())
    }
}

fn build_impexp_pass_based<C>(args: &mut Vec<String>) -> Result<Box<dyn Command>, ()>
    where C: 'static + From::<(String, Box::<dyn EncryptionStrategy>, bool)> + Command {
    match args.len() >= 1 {
        true => {
            let data = unpack_one(args, 0);
            let clear = if args.len() > 1 { &unpack_one(args, 1) == CLEAR_FLAG } else { false };
            let strategy = Box::new(encryption_strategy::PassBased);
            let cmd = Box::new(C::from((data, strategy, clear)));
            Ok(cmd)
        },
        false => Err(())
    }
}
