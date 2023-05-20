use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub struct GpioError(pub String);

impl Display for GpioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Error for GpioError {}
