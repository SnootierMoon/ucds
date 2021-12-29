use std::{error, fmt};

#[derive(Debug)]
pub struct UcdFileParseError;

impl fmt::Display for UcdFileParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse UnicodeData")?;
        Ok(())
    }
}

impl error::Error for UcdFileParseError {}
