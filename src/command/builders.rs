use super::{
    definitions::*,
    tools::encryption_strategy::{self, EncryptionStrategy},
};

const CLEAR_FLAG: &str = "-c";
const SINGLE_KEY_USAGE: &str = "<key>";
const KEY_LIST_USAGE: &str = "<key> [, <key>, <key>, ... ]";
const IMPORT_PATH: &str = "<from_path>";
const EXPORT_PATH: &str = "<export_path>";
const KEY_PATH: &str = "<key_path>";
const FLAG: &str = "[-c] (c - for clear)";

pub trait CmdBuilder {
    fn build(&self, args: Vec<String>) -> Result<Box<dyn Command>, ()>;
    fn cmd_usage(&self) -> String;
}

pub struct ListBuilder;
impl CmdBuilder for ListBuilder {
    fn build(&self, _args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        Ok(Box::new(List))
    }

    fn cmd_usage(&self) -> String {
        String::new()
    }
}

pub struct ShowBuilder;
impl CmdBuilder for ShowBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_from_one::<Show>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from(SINGLE_KEY_USAGE)
    }
}

pub struct AddBuilder;
impl CmdBuilder for AddBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_from_one::<Add>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from(SINGLE_KEY_USAGE)
    }
}

pub struct RemoveBuilder;
impl CmdBuilder for RemoveBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_from_one::<Remove>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from(SINGLE_KEY_USAGE)
    }
}

pub struct UpdateBuilder;
impl CmdBuilder for UpdateBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_from_one::<Update>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from(SINGLE_KEY_USAGE)
    }
}

pub struct RSAExportBuilder;
impl CmdBuilder for RSAExportBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_impexp_key_based::<Export>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        format!("{} {} {}", EXPORT_PATH, KEY_PATH, FLAG)
    }
}

pub struct RSAImportBuilder;
impl CmdBuilder for RSAImportBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_impexp_key_based::<Import>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        format!("{} {} {}", IMPORT_PATH, KEY_PATH, FLAG)
    }
}

pub struct PassBasedExportBuilder;
impl CmdBuilder for PassBasedExportBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_impexp_pass_based::<Export>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        format!("{} {}", EXPORT_PATH, FLAG)
    }
}

pub struct PassBasedImportBuilder;
impl CmdBuilder for PassBasedImportBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_impexp_pass_based::<Import>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        format!("{} {}", IMPORT_PATH, FLAG)
    }
}

pub struct RenameBuilder;
impl CmdBuilder for RenameBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_from_two::<Rename>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from("<old_key> <new_key>")
    }
}

pub struct ClearBuilder;
impl CmdBuilder for ClearBuilder {
    fn build(&self, mut _args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        Ok(Box::new(Clear))
    }

    fn cmd_usage(&self) -> String {
        String::new()
    }
}

pub struct CopyBuilder;
impl CmdBuilder for CopyBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_from_one::<Copy>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from(SINGLE_KEY_USAGE)
    }
}

pub struct MultiAddBuilder;
impl CmdBuilder for MultiAddBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_from_list::<MultiAdd>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from(KEY_LIST_USAGE)
    }
}

pub struct MultiRemoveBuilder;
impl CmdBuilder for MultiRemoveBuilder {
    fn build(&self, args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        if args.len() < 1 { return Err(()); }
        Ok(Box::new(MultiRemove{ keys: args }))
    }

    fn cmd_usage(&self) -> String {
        String::from(KEY_LIST_USAGE)
    }
}

pub struct MultiUpdateBuilder;
impl CmdBuilder for MultiUpdateBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_from_list::<MultiUpdate>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from(KEY_LIST_USAGE)
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
