use std::error;
use std::fmt;

#[derive(Debug)]
pub struct ReservedImageError {
    pub image_name: String,
    pub event: String,
}

impl fmt::Display for ReservedImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "cannot {} because {} image is reserved instance",
            self.event, self.image_name
        )
    }
}

impl error::Error for ReservedImageError {}

#[derive(Debug)]
pub struct UnknownImageNameError {
    pub image_name: String,
}

impl fmt::Display for UnknownImageNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown image name: {}", self.image_name)
    }
}

impl error::Error for UnknownImageNameError {}
