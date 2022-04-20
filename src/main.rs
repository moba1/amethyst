mod scriptlets;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(parse(try_from_str=parse_path))]
    config_directory: std::path::PathBuf,
}

fn parse_path(s: &str) -> Result<std::path::PathBuf, String> {
    let mut path = std::path::PathBuf::new();
    path.push(s);
    Ok(path)
}

fn main() {
    let args = Args::parse();
    let config_directory = &args.config_directory;
    match scriptlets::load(config_directory) {
        Ok(_) => {},
        Err(error) => println!("cannot load configs from {} [{}]", config_directory.display(), error),
    };
}
