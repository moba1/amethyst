mod command;
mod config;
mod result;

use std::process;

use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Commands {
    Build { config_directory: String },
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

fn main() {
    let args = Args::parse();
    let command = match &args.command {
        Commands::Build { config_directory } => || command::build(config_directory.clone()),
    };
    if let Err(err) = command() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
