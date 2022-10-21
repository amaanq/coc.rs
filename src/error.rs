use reqwest::header::InvalidHeaderValue;

#[derive(Debug)]
pub enum APIError {
    /// API hasn't been initialized yet (logging in + making keys).
    ClientNotReady,
    /// Failed to query the current ip address.
    FailedGetIP(String),
    /// Failed to login to an account, either due to invalid credentials or a server error.
    LoginFailed(String),
    /// Reqwest error
    RequestFailed(reqwest::Error),
    /// Invalid Header, should never happen
    InvalidHeader(InvalidHeaderValue),
    /// Failed to parse a URL, should never happen
    BadUrl(url::ParseError),
    /// Status code of 400
    BadParameters,
    /// Status code of 403
    AccessDenied,
    /// Status code of 404
    NotFound,
    /// Status code of 429
    RequestThrottled,
    /// Status code of 500
    UnknownError,
    /// Status code of 503
    InMaintenance,
    /// All other cases (edge cases/unknown status codes)
    BadResponse(String, reqwest::StatusCode),
    /// From malformed cursors or using invalid leagues
    InvalidParameters(String),
    InvalidTag(String),

    EventFailure(String),
}

impl std::error::Error for APIError {}

impl From<logic_long::LogicLongError> for APIError {
    fn from(e: logic_long::LogicLongError) -> Self {
        Self::InvalidTag(e.to_string())
    }
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

impl std::fmt::Display for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ClientNotReady => write!(f, "Client not ready"),
            Self::FailedGetIP(e) => write!(f, "Failed to get IP address: {e}"),
            Self::LoginFailed(e) => write!(f, "Failed to login: {e}"),
            Self::RequestFailed(e) => write!(f, "Request failed: {e}"),
            Self::InvalidHeader(e) => write!(f, "Invalid header: {e}"),
            Self::BadUrl(e) => write!(f, "Bad URL: {e}"),
            Self::BadParameters => write!(f, "Bad parameters"),
            Self::AccessDenied => write!(f, "Access denied"),
            Self::NotFound => write!(f, "Not found"),
            Self::RequestThrottled => write!(f, "Request throttled"),
            Self::UnknownError => write!(f, "Unknown error"),
            Self::InMaintenance => write!(f, "In maintenance"),
            Self::BadResponse(e, s) => write!(f, "Bad response: {e} ({s})"),
            Self::InvalidParameters(e) => write!(f, "Invalid parameters: {e}"),
            Self::InvalidTag(e) => write!(f, "Invalid tag: {e}"),
            Self::EventFailure(e) => write!(f, "Event failure: {e}"),
        }
    }
}
