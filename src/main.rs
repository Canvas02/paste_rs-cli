// Copyright 2022 Canvas02 <Canvas02@protonmail.com>
// SPDX-License-Identifier: MIT

#![deny(unused)]

mod api;
mod cli;
mod error;

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

                    std::process::exit(0);
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

// => A main for testing
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     Ok(())
// }
