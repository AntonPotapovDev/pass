use crate::command::builders;

pub const CMD_ADD: &str = "add";
pub const CMD_REMOVE: &str = "rm";
pub const CMD_UPDATE: &str = "update";
pub const CMD_LIST: &str = "list";
pub const CMD_SHOW: &str = "show";

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
