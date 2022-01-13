// Copyright 2022 Canvas02.
// SPDX-License-Identifier: MIT

// ! Used in development never turn on for a production build
#![allow(dead_code)]
#![allow(unused_variables)]
// ! enable for production build
// #![deny(dead_code)]
// #![deny(unused_variables)]

mod cache;
mod paste;

use crate::paste::Url;
use clap::{AppSettings, Parser, Subcommand};
use std::fs;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Get { val } => {
            let url = match Url::new(&*val) {
                Ok(val) => val,
                Err(err) => {
                    eprintln!("ERROR: {:#?}", err);
                    std::process::exit(-1);
                }
            };

            let res = match url.make_request().await {
                Ok(val) => val,
                Err(err) => {
                    eprintln!("ERROR: {:#?}", err);
                    std::process::exit(-1);
                }
            };

            // TODO: Implement cache
            // crate::cache::new_cache()

            println!(
                "====\t{}\t======================================================",
                url
            );
            println!("======================================================================================");
            println!("");
            println!("{}", res);
            println!("");
            println!("======================================================================================");
            println!("======================================================================================");
            std::process::exit(0);
        }
        Commands::New { file } => {
            let data = match fs::read_to_string(file) {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("ERROR: {:#?}", err);
                    std::process::exit(-1);
                }
            };

            let url = match paste::new_paste(data).await {
                Ok(res) => res,
                Err(err) => {
                    eprintln!("ERROR: {:#?}", err);
                    std::process::exit(-1);
                }
            };

            println!("Successfully created new paste at {}", url);
        }
    }
}

#[derive(Parser)]
#[clap(name = "paste-rs")]
#[clap(about = "a simple cli tool for https://paste.rs")]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // Command to get a paste
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    #[clap(about = "Get a paste")]
    Get {
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

// => A main for testing
// #[tokio::main]
// async fn main() {
//     let mut url = match crate::paste::Url::new("osx") {
//         Ok(val) => val,
//         Err(err) => {
//             eprintln!("Error: {:#?}", err);
//             std::process::exit(-1);
//         }
//     };

//     let id = url.get_id();
//     dbg!(id);
// }
