//! Library entry point exposing Telescope command handlers.

pub mod commands;
pub mod core;
pub mod error;
pub mod gateway;
pub mod model;
pub mod storage;

pub use commands::init;
pub use storage::InitReport;
