use std::error;

pub type Result<T> = std::result::Result<T, BoxedError>;
pub type BoxedError = Box<dyn error::Error + Send + Sync + 'static>;
