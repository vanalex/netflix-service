use std::fmt;

/// Custom error type for TMDB API operations
#[derive(Debug, Clone)]
pub enum TmdbError {
    /// Network-related errors (timeouts, connection failures, etc.)
    NetworkError(String),

    /// JSON parsing/deserialization errors
    ParseError(String),

    /// API rate limit exceeded (HTTP 429)
    RateLimitExceeded,

    /// Resource not found (HTTP 404)
    NotFound,

    /// Unauthorized access (HTTP 401)
    Unauthorized,

    /// Server error (HTTP 5xx)
    ServerError(u16),

    /// Invalid request (HTTP 400)
    BadRequest(String),

    /// Unknown error with status code
    Unknown(u16, String),
}

impl fmt::Display for TmdbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TmdbError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            TmdbError::ParseError(msg) => write!(f, "Failed to parse response: {}", msg),
            TmdbError::RateLimitExceeded => write!(f, "API rate limit exceeded"),
            TmdbError::NotFound => write!(f, "Resource not found"),
            TmdbError::Unauthorized => write!(f, "Unauthorized: Invalid or missing API key"),
            TmdbError::ServerError(code) => write!(f, "Server error: {}", code),
            TmdbError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            TmdbError::Unknown(code, msg) => write!(f, "Unknown error ({}): {}", code, msg),
        }
    }
}

impl std::error::Error for TmdbError {}

impl From<reqwest::Error> for TmdbError {
    fn from(error: reqwest::Error) -> Self {
        TmdbError::NetworkError(error.to_string())
    }
}

impl From<serde_json::Error> for TmdbError {
    fn from(error: serde_json::Error) -> Self {
        TmdbError::ParseError(error.to_string())
    }
}

impl TmdbError {
    /// Creates a TmdbError from an HTTP status code
    pub fn from_status(status: reqwest::StatusCode, body: String) -> Self {
        match status.as_u16() {
            400 => TmdbError::BadRequest(body),
            401 => TmdbError::Unauthorized,
            404 => TmdbError::NotFound,
            429 => TmdbError::RateLimitExceeded,
            500..=599 => TmdbError::ServerError(status.as_u16()),
            code => TmdbError::Unknown(code, body),
        }
    }

    /// Returns true if this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            TmdbError::NetworkError(_)
            | TmdbError::RateLimitExceeded
            | TmdbError::ServerError(_)
        )
    }
}
