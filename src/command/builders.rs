use super::{Command, encryption_strategy::{self, EncryptionStrategy}};

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
        build_from_two::<super::Add>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from("<key> <password>")
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
        build_from_two::<super::Update>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from("<key> <new_password>")
    }
}

pub struct RSAExportBuilder;
impl CmdBuilder for RSAExportBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_impexp::<super::Export, encryption_strategy::KeyBased>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from("<export_path> <key_path>")
    }
}

pub struct RSAImportBuilder;
impl CmdBuilder for RSAImportBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_impexp::<super::Import, encryption_strategy::KeyBased>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from("<import_path> <key_path>")
    }
}

pub struct PassBasedExportBuilder;
impl CmdBuilder for PassBasedExportBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_impexp::<super::Export, encryption_strategy::PassBased>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from("<export_path> <password>")
    }
}

pub struct PassBasedImportBuilder;
impl CmdBuilder for PassBasedImportBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_impexp::<super::Import, encryption_strategy::PassBased>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from("<import_path> <password>")
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
        build_from_list_with_tail::<super::MultiAdd>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from("<key> [, <key>, <key>, ... ] <pass>")
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
        build_from_list_with_tail::<super::MultiUpdate>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from("<key> [, <key>, <key>, ... ] <pass>")
    }
}

fn unpack_two(args: &mut Vec<String>) -> (String, String) {
    let first = std::mem::replace(&mut args[0], String::new());
    let second = std::mem::replace(&mut args[1], String::new());
    (first, second)
}

fn build_from_one<T>(args: &mut Vec<String>) -> Result<Box<dyn Command>, ()>
    where T: 'static + From::<String> + Command {
    match args.len() >= 1 {
        true => Ok(Box::new(T::from(std::mem::replace(&mut args[0], String::new())))),
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

fn build_from_list_with_tail<T>(args: &mut Vec<String>) -> Result<Box<dyn Command>, ()>
    where T: 'static + From::<(Vec<String>, String)> + Command {
    if args.len() < 2 { return Err(()); }

    let mut keys = Vec::with_capacity(args.len() - 1);

    let mut i = 0;
    loop {
        if i == args.len() - 1 { break; }
        let key = std::mem::replace(&mut args[i], String::new());
        keys.push(key);
        i += 1;
    }

    Ok(Box::new(T::from((keys, args.last().unwrap().clone()))))
}

fn build_impexp<C, S>(args: &mut Vec<String>) -> Result<Box<dyn Command>, ()>
    where C: 'static + From::<(String, Box::<dyn EncryptionStrategy>)> + Command,
          S: 'static + From::<String> + EncryptionStrategy {
    match args.len() >= 2 {
        true => {
            let (data, key) = unpack_two(args);
            let strategy = Box::new(S::from(key));
            let cmd = Box::new(C::from((data, strategy)));
            Ok(cmd)
        },
        false => Err(())
    }
}
