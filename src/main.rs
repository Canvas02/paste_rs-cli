// Copyright 2022 Canvas02 <Canvas02@protonmail.com>
// SPDX-License-Identifier: MIT

#![deny(dead_code)]
#![deny(unused_variables)]

#[macro_use]
extern crate anyhow;

pub mod api;
mod cli;

use crate::api::Paste;
use crate::cli::{Cli, Commands};
use clap::Parser;
use std::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

                    std::process::exit(0);
                }
            }
        }
        Commands::New { file } => {
            let data = fs::read_to_string(file)?;

            let paste = Paste::new(data).await?;
            println!("Successfully created new paste at {}", paste.get_url());
        }
    }
    Ok(())
}

// => A main for testing
// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     Ok(())
// }
