use std::path;
use std::convert;
use std::fmt;
use std::error;

mod config;

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
    command: Commands
}

#[derive(Debug)]
struct ConfigLoadError {
    original_error: Box<dyn error::Error + Send + Sync + 'static>
}

impl fmt::Display for ConfigLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cannot load configuration file [{}]", self.original_error)
    }
}

impl error::Error for ConfigLoadError {}

fn load_config<P>(config_directory: P) -> Result<config::Config, ConfigLoadError>
    where P: convert::AsRef<path::Path>
{
    match config::load(&config_directory) {
        Ok(config) => return Ok(config),
        Err(error) => Err(ConfigLoadError {
            original_error: error,
        }),
    }
}

fn main() {
    let args = Args::parse();
    let command = match &args.command {
        Commands::Build { config_directory } => || build(config_directory.clone())
    };
    if let Err(err) = command() {
        eprintln!("{}", err);
    }
}

fn build<P>(config_directory: P) -> Result<(), Box<dyn error::Error>>
    where P: convert::AsRef<path::Path>
{
    let config = load_config(config_directory)?;
    for image in config.images() {
        println!("{:?}", image.slurp_scriptlets());
    };
    Ok(())
}
