pub mod builders;
pub mod resolver;

mod cmd_impl;

pub use cmd_impl::*;

use crate::context::Context;

pub trait Command {
    fn execute(&self, model: &mut Context);
}
