use crate::command::{self, Command};

const CMD_ADD: &str = "add";
const CMD_REMOVE: &str = "rm";
const CMD_UPDATE: &str = "update";
const CMD_LIST: &str = "list";

pub fn parse_command(cmd: &str, mut args: Vec<String>) -> Result<Box<dyn Command>, ()> {
    match cmd {
        CMD_ADD => {
            if args.len() < 2 { Err(()) }
            else {
                let key = std::mem::replace(&mut args[0], String::new());
                let pass = std::mem::replace(&mut args[1], String::new());
                Ok(Box::new(command::Add{ key, pass }))
            }
        },
        CMD_REMOVE => {
            if args.len() < 1 { Err(()) }
            else {
                let key = std::mem::replace(&mut args[0], String::new());
                Ok(Box::new(command::Remove{ key }))
            }
        },
        CMD_UPDATE => {
            if args.len() < 2 { Err(()) }
            else {
                let key = std::mem::replace(&mut args[0], String::new());
                let pass = std::mem::replace(&mut args[1], String::new());
                Ok(Box::new(command::Update{ key, pass }))
            }
        },
        CMD_LIST => {
            Ok(Box::new(command::List))
        },
        _ => Err(()),
    }
}
