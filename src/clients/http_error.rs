use reqwest::Error as ReqwestError;

#[derive(Debug)]
pub enum HttpError {
    UrlParseError,
    ReqwestError { status_code: u16, message: String, error_type: Option<String> },
    ResponseError { message: String },
}

impl From<ReqwestError> for HttpError {
    fn from(error: ReqwestError) -> Self {
        if error.url().is_some() {
            HttpError::UrlParseError
        } else {
            let message = error.to_string();
            let error_type = if error.is_status() {
                HttpError::ReqwestError {
                    status_code: error.status().map_or(0, |s| s.as_u16()),
                    message,
                    error_type: None,
                }
            } else {
                HttpError::ResponseError {
                    message,
                }
            };
            error_type
        }
    }
}