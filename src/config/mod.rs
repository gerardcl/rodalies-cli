//! # config
//!
//! The `config` module provides CLI utilities such as the CLI arguments parser and the Pretty table.

/// `cli` provides the methods to initialize the CLI input (args) and output (table).
pub mod cli;

/// `check` provides the methods to check and inform about latest published online version of the `rodalies-cli` and the one being used by the user.
pub mod check;
