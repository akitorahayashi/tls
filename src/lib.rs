//! Library entry point exposing Telescope command handlers.

pub mod commands;
pub mod error;
pub mod gateway;
pub mod model;
mod scaffold;

pub use commands::init;
pub use scaffold::{InitReport, ProjectLayout};
