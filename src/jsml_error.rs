use std::{error, fmt};

#[derive(Debug)]
pub struct JsmlError {
    pub details: String,
}

impl JsmlError {
    pub fn new(msg: &str) -> Self {
        Self {
            details: msg.to_string(),
        }
    }
}

impl error::Error for JsmlError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl fmt::Display for JsmlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl From<JsmlError> for std::io::Error {
    fn from(err: JsmlError) -> Self {
        Self::new(std::io::ErrorKind::Other, err.details.as_str())
    }
}

impl From<std::io::Error> for JsmlError {
    fn from(err: std::io::Error) -> Self {
        Self::new(&err.to_string())
    }
}
