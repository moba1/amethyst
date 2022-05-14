use std::error;
use std::fmt;

#[derive(Debug)]
pub struct HttpError {
    pub status_code: reqwest::StatusCode,
    pub message: String,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.message, self.status_code)
    }
}

impl error::Error for HttpError {}
