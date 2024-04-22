#![warn(clippy::all)]
mod vm;
pub use vm::VM;
mod args;
pub use args::Args;