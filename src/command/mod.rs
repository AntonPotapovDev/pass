pub mod builders;
pub mod resolver;

mod commands;
mod encryption_strategy;
mod dialog;
mod msg;

pub use commands::*;

use crate::context::Context;

pub trait Command {
    fn execute(&self, model: &mut Context);
}
