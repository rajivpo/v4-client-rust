use reqwest::Error as ReqwestError;
use std::fmt;

#[derive(Debug)]
pub struct HttpError {
    status_code: u16,
    message: String,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HTTP ERROR {}: {}", self.status_code, self.message)
    }
}

impl From<ReqwestError> for HttpError {
    fn from(error: ReqwestError) -> Self {
        Self {
            status_code: error.status().map_or(0, |s| s.as_u16()),
            message: error.to_string(),
        }
    }
}