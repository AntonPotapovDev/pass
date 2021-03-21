use crate::command::builders;

pub const CMD_ADD: &str = "add";
pub const CMD_REMOVE: &str = "rm";
pub const CMD_UPDATE: &str = "update";
pub const CMD_LIST: &str = "list";
pub const CMD_SHOW: &str = "show";
pub const CMD_EXPORT: &str = "export";
pub const CMD_IMPORT: &str = "import";

pub fn resolve_command(cmd: &str) -> Result<Box<dyn builders::CmdBuilder>, ()> {
    match cmd {
        CMD_ADD => Ok(Box::new(builders::AddBuilder)),
        CMD_REMOVE => Ok(Box::new(builders::RemoveBuilder)),
        CMD_UPDATE => Ok(Box::new(builders::UpdateBuilder)),
        CMD_LIST => Ok(Box::new(builders::ListBuilder)),
        CMD_SHOW => Ok(Box::new(builders::ShowBuilder)),
        CMD_EXPORT => Ok(Box::new(builders::ExportBuilder)),
        CMD_IMPORT => Ok(Box::new(builders::ImportBuilder)),
        _ => Err(()),
    }
}
