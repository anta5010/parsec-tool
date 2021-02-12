// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Base CLI implementation.

use crate::common::{PROJECT_AUTHOR, PROJECT_DESC, PROJECT_NAME, PROJECT_VERSION};
use crate::subcommands::Subcommand;
use structopt::StructOpt;

/// Struct representing the command-line interface of parsec-tool.
#[derive(Debug, StructOpt)]
#[structopt(name=PROJECT_NAME, about=PROJECT_DESC, author=PROJECT_AUTHOR, version=PROJECT_VERSION)]
pub struct ParsecToolApp {
    /// The ID of the provider to target for the command. Will use the default provider if not specified.
    #[structopt(short = "p", long = "provider")]
    pub provider: Option<u8>,

    /// The subcommand -- e.g., ping.
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}
