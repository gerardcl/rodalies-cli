//! # rodalies-cli
//!
//! `rodalies-cli` library is the collection of methods required for the `rodalies-cli` CLI binary.
//! CLI for searching train timetables of the trains of Rodalies de la Generalitat de Catalunya.
//!
//! Learn more about `rodalies-cli` code and usage at the main [project page](https://github.com/gerardcl/rodalies-cli).

/// `config` module handles the initialization of `rodalies-cli` CLI.
pub mod config;

/// `rodalies` module contains the business logic of the whole `rodalies-cli` tool.
pub mod rodalies;
