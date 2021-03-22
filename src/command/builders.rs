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

pub struct ExportBuilder;
impl CmdBuilder for ExportBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_from_two::<super::Export>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from("<export_path> <key_path>")
    }
}

pub struct ImportBuilder;
impl CmdBuilder for ImportBuilder {
    fn build(&self, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
        build_from_two::<super::Import>(&mut args)
    }

    fn cmd_usage(&self) -> String {
        String::from("<import_path> <key_path>")
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

fn build_from_one<T: 'static + From::<String> + Command>(args: &mut Vec<String>) -> Result<Box<dyn Command>, ()> {
    match args.len() >= 1 {
        true => Ok(Box::new(T::from(std::mem::replace(&mut args[0], String::new())))),
        false => Err(()),
    }
}

fn build_from_two<T: 'static + From::<(String, String)> + Command>(args: &mut Vec<String>) -> Result<Box<dyn Command>, ()> {
    match args.len() >= 1 {
        true => {
            let first = std::mem::replace(&mut args[0], String::new());
            let second = std::mem::replace(&mut args[1], String::new());
            Ok(Box::new(T::from((first, second))))
        }
        false => Err(()),
    }
}
