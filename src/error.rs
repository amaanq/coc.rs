use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum APIError {
    /// API hasn't been initialized yet (logging in + making keys).
    #[error("Client hasn't been initialized yet (logging in + making keys).")]
    ClientNotReady,
    /// Failed to query the current ip address.
    #[error("Failed to get IP address: {0}")]
    FailedGetIP(String),
    /// Failed to login to an account, either due to invalid credentials or a server error.
    #[error("Failed to login: {0}")]
    LoginFailed(String),
    /// Reqwest error
    #[error("Request failed: {0}")]
    RequestFailed(reqwest::Error),
    /// Invalid Header, should never happen
    #[error("Invalid header: {0}")]
    InvalidHeader(InvalidHeaderValue),
    /// Failed to parse a URL, should never happen
    #[error("Failed to parse URL: {0}")]
    BadUrl(url::ParseError),
    /// Status code of 400
    #[error("Bad parameters")]
    BadParameters,
    /// Status code of 403
    #[error("Access denied")]
    AccessDenied,
    /// Status code of 404
    #[error("Not found")]
    NotFound,
    /// Status code of 429
    #[error("Request Throttled")]
    RequestThrottled,
    /// Status code of 500
    #[error("Unknown error (500)")]
    UnknownError,
    /// Status code of 503
    #[error("In maintenance")]
    InMaintenance,
    /// All other cases (edge cases/unknown status codes)
    #[error("Bad response: {0}")]
    BadResponse(String, reqwest::StatusCode),
    /// From malformed cursors or using invalid leagues
    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),
    #[error("Invalid tag: {0}")]
    InvalidTag(String),
    /// General event failure
    #[error("Event failure: {0}")]
    EventFailure(String),
}

impl From<reqwest::Error> for APIError {
    fn from(e: reqwest::Error) -> Self {
        Self::RequestFailed(e)
    }
}

impl From<url::ParseError> for APIError {
    fn from(e: url::ParseError) -> Self {
        Self::BadUrl(e)
    }
}

impl From<InvalidHeaderValue> for APIError {
    fn from(e: InvalidHeaderValue) -> Self {
        Self::InvalidHeader(e)
    }
}
