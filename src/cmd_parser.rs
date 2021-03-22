use crate::command::builders;

pub const CMD_ADD: &str = "add";
pub const CMD_REMOVE: &str = "rm";
pub const CMD_UPDATE: &str = "update";
pub const CMD_LIST: &str = "list";
pub const CMD_SHOW: &str = "show";
pub const CMD_EXPORT: &str = "export";
pub const CMD_IMPORT: &str = "import";
pub const CMD_RENAME: &str = "rename";
pub const CMD_CLEAR: &str = "clear";
pub const CMD_COPY: &str = "copy";

pub fn resolve_command(cmd: &str) -> Result<Box<dyn builders::CmdBuilder>, ()> {
    match cmd {
        CMD_ADD => Ok(Box::new(builders::AddBuilder)),
        CMD_REMOVE => Ok(Box::new(builders::RemoveBuilder)),
        CMD_UPDATE => Ok(Box::new(builders::UpdateBuilder)),
        CMD_LIST => Ok(Box::new(builders::ListBuilder)),
        CMD_SHOW => Ok(Box::new(builders::ShowBuilder)),
        CMD_EXPORT => Ok(Box::new(builders::ExportBuilder)),
        CMD_IMPORT => Ok(Box::new(builders::ImportBuilder)),
        CMD_RENAME => Ok(Box::new(builders::RenameBuilder)),
        CMD_CLEAR => Ok(Box::new(builders::ClearBuilder)),
        CMD_COPY => Ok(Box::new(builders::CopyBuilder)),
        _ => Err(()),
    }
}
