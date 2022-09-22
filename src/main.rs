// Copyright 2022 Canvas02 <Canvas02@protonmail.com>
// SPDX-License-Identifier: MIT

#![deny(unused)]

mod api;

use crate::api::Paste;
use crate::cli::{Cli, Commands};
use clap::Parser;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match &args.command {
        Commands::Get { val, output } => {
            let paste = Paste::from(val)?;

            let res = paste.get().await?;

            match output {
                Some(path) => {
                    fs::write(path, res)?;
                    println!(
                        "Successfully wrote {} to {}",
                        paste.get_url(),
                        path.display()
                    )
                }
                None => {
                    println!(
                        "====\t{}\t======================================================",
                        paste.get_url()
                    );
                    println!("");
                    println!("{}", res);
                    println!("");
                    println!("======================================================================================");

                    return Ok(());
                }
            }
        }
        Commands::New { file } => {
            let data = fs::read_to_string(file)?;

            let paste = Paste::new(data).await?;
            dbg!(&paste.id);
            match paste.status_code {
                Some(stat) => {
                    if stat.as_u16() == 201 {
                        println!("Successfully created new paste at {}", paste.get_url());
                    } else if stat.as_u16() == 206 {
                        println!("FILE TOO BIG");
                        println!("Pastialy created new paste at {}", paste.get_url());
                    } else {
                        unreachable!()
                    }
                }
                None => unreachable!(),
            }
        }
    }
    Ok(())
}

// Cli
mod cli {
    // Copyright 2022 Canvas02 <Canvas02@protonmail.com>
    // SPDX-License-Identifier: MIT

    use clap::{Parser, Subcommand};
    use std::path::PathBuf;

    #[derive(Debug, Parser)]
    #[clap(name = "paste-rs")]
    #[clap(about, author, version)]
    pub struct Cli {
        #[clap(subcommand)]
        pub command: Commands,
    }

    #[derive(Debug, Subcommand)]
    pub enum Commands {
        // Command to get a paste
        // TODO(Canvas02): migrate to 'Command::arg_required_else_help' + 'Command::is_arg_required_else_help_set'
        // #[clap(setting(AppSettings::ArgRequiredElseHelp))]
        // #[clap(about = "Get a paste")]
        #[clap(arg_required_else_help = true)]
        Get {
            // TODO(Canvas02): migrate from 'from_os_str' to 'vaule_parser'
            // #[clap(short, long, parse(from_os_str))]
            #[clap(short = 'o', value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
            output: Option<PathBuf>,
            #[clap(required = true)]
            val: String,
        },

        // Command to make a paste
        // #[clap(setting(AppSettings::ArgRequiredElseHelp))]
        #[clap(arg_required_else_help = true)]
        #[clap(about = "Make a new paste")]
        New {
            #[clap(required = true)]
            file: String,
        },
    }
}
