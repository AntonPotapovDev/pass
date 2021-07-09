use super::definitions::*;


const CLEAR_FLAG: &str = "-c";
const SINGLE_KEY_USAGE: &str = "<key>";
const KEY_LIST_USAGE: &str = "<key> [, <key>, <key>, ... ]";
const IMPORT_PATH: &str = "[<from_path>]";
const EXPORT_PATH: &str = "[<export_path>]";
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

pub struct ExportBuilder;
impl CmdBuilder for ExportBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_impexp::<Export>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        format!("{} {}", EXPORT_PATH, FLAG)
    }
}

pub struct ImportBuilder;
impl CmdBuilder for ImportBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_impexp::<Import>(&mut args)
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
    fn build(&self, args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_from_list::<MultiAdd>(args)
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
    fn build(&self, args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_from_list::<MultiUpdate>(args)
    }

    fn cmd_usage(&self) -> String {
        String::from(KEY_LIST_USAGE)
    }
}

pub struct PasteBuilder;
impl CmdBuilder for PasteBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_from_one::<Paste>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from(SINGLE_KEY_USAGE)
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

fn build_from_list<T>(args: Vec<String>) -> Result<Box<dyn Command>, ()>
    where T: 'static + From::<Vec<String>> + Command {
    match args.len() < 1 {
        true => Err(()),
        false => Ok(Box::new(T::from(args))), 
    } 
}

fn build_impexp<T>(args: &mut Vec<String>) -> Result<Box<dyn Command>, ()>
    where T: 'static + From::<(String, bool)> + Command {
    if args.len() > 2 {
        return Err(());
    }

    let mut flags_count = 0;
    let mut path_idxs = vec![];

    for (idx, arg) in args.iter().enumerate() {
        match arg.as_str() {
            CLEAR_FLAG => flags_count += 1,
            _ => path_idxs.push(idx),
        }
    }

    let clear = match flags_count {
        0 => false,
        1 => true,
        _ => return Err(()),
    };

    let path = match path_idxs.len() {
        0 => String::new(),
        1 => unpack_one(args, path_idxs[0]),
        _ => return Err(()),
    };

    Ok(Box::new(T::from((path, clear))))
}
