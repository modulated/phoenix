#![warn(clippy::all)]
mod vm;
pub use vm::VM;
mod args;
pub use args::Args;
mod types;
mod util;
pub use vm::StatusRegister;
mod constants;
pub use constants::*;