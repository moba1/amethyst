use std::convert;
use std::path;

mod config;
mod result;

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

fn load_config<P>(config_directory: P) -> result::Result<config::Config>
where
    P: convert::AsRef<path::Path>,
{
    match config::load(&config_directory) {
        Ok(config) => Ok(config),
        err => err,
    }
}

fn main() {
    let args = Args::parse();
    let command = match &args.command {
        Commands::Build { config_directory } => || build(config_directory.clone()),
    };
    if let Err(err) = command() {
        eprintln!("{}", err);
    }
}

fn build<P>(config_directory: P) -> result::Result<()>
where
    P: convert::AsRef<path::Path>,
{
    let config = load_config(config_directory)?;
    for image in config.images() {
        match image.slurp_scriptlets() {
            Ok(scriptlets) => println!("{:?}", scriptlets),
            Err(err) => return Err(err),
        }
    }
    Ok(())
}
