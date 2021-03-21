use crate::command::builders;

const CMD_ADD: &str = "add";
const CMD_REMOVE: &str = "rm";
const CMD_UPDATE: &str = "update";
const CMD_LIST: &str = "list";
const CMD_SHOW: &str = "show";

pub fn resolve_command(cmd: &str) -> Result<Box<dyn builders::CmdBuilder>, ()> {
    match cmd {
        CMD_ADD => Ok(Box::new(builders::AddBuilder)),
        CMD_REMOVE => Ok(Box::new(builders::RemoveBuilder)),
        CMD_UPDATE => Ok(Box::new(builders::UpdateBuilder)),
        CMD_LIST => Ok(Box::new(builders::ListBuilder)),
        CMD_SHOW => Ok(Box::new(builders::ShowBuilder)),
        _ => Err(()),
    }
}
