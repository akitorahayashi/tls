//! Library entry point exposing Telescope command handlers.

pub mod commands;
pub mod error;
mod scaffold;

pub use commands::init;
pub use scaffold::{InitReport, ProjectLayout};
