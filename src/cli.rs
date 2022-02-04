// Copyright 2022 Canvas02 <Canvas02@protonmail.com>
// SPDX-License-Identifier: MIT

use clap::{AppSettings, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(name = "paste-rs")]
#[clap(about = "a simple cli tool for https://paste.rs")]
#[clap(author, version)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // Command to get a paste
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    #[clap(about = "Get a paste")]
    Get {
        #[clap(short, long, parse(from_os_str))]
        output: Option<PathBuf>,
        #[clap(required = true)]
        val: String,
    },

    // Command to make a paste
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    #[clap(about = "Make a new paste")]
    New {
        #[clap(required = true)]
        file: String,
    },
}
