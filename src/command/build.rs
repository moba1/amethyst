use crate::config;
use crate::result;
use std::convert;
use std::env;
use std::error;
use std::fmt;
use std::path;

#[derive(Debug)]
struct UnchangedWorkingDirectory {
    to: String,
    original_error: result::BoxedError,
}

impl fmt::Display for UnchangedWorkingDirectory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "cannot change directory to {:?} [{}]",
            self.to, self.original_error
        )
    }
}

impl error::Error for UnchangedWorkingDirectory {}

pub fn build<P>(config_directory: P) -> result::Result<()>
where
    P: convert::AsRef<path::Path>,
{
    if let Err(error) = env::set_current_dir(&config_directory) {
        return Err(Box::new(UnchangedWorkingDirectory {
            to: config_directory.as_ref().to_string_lossy().to_string(),
            original_error: Box::new(error),
        }));
    }

    let config = config::build()?;
    match serde_yaml::to_string(&config) {
        Ok(config) => println!("{}", config),
        Err(err) => eprintln!("error occured: {}", err),
    }
    Ok(())
}
