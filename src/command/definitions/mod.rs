mod basic;
mod impexp;
mod misc;

pub use basic::*;
pub use impexp::*;
pub use misc::*;

use super::tools;

pub trait Command {
    fn execute(self: Box<Self>, model: &mut crate::context::Context);
}
