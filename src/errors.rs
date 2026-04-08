use std::fmt;

/// Error code constants returned by the Rollover API.
pub mod error_code {
    pub const INVALID_API_KEY: &str = "invalid_api_key";
    pub const UNAUTHORIZED: &str = "unauthorized";
    pub const RATE_LIMIT: &str = "rate_limit_exceeded";
    pub const NOT_FOUND: &str = "not_found";
    pub const INSUFFICIENT_CREDITS: &str = "insufficient_credits";
    pub const VALIDATION: &str = "validation_error";
}

/// An error returned by the Rollover SDK.
#[derive(Debug)]
pub enum RolloverError {
    /// An error response from the Rollover API.
    Api {
        status: u16,
        code: String,
        message: String,
    },
    /// An HTTP transport error.
    Http(reqwest::Error),
    /// A configuration error (e.g. missing API key).
    Config(String),
}

impl RolloverError {
    /// Returns true if the error is likely transient and the request could
    /// succeed on retry, such as rate limits (429) or server errors (5xx).
    pub fn temporary(&self) -> bool {
        match self {
            RolloverError::Api { status, .. } => *status == 429 || *status >= 500,
            _ => false,
        }
    }

    /// Returns true if this is an API error with the given error code.
    pub fn is_code(&self, code: &str) -> bool {
        match self {
            RolloverError::Api { code: c, .. } => c == code,
            _ => false,
        }
    }
}

impl fmt::Display for RolloverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RolloverError::Api {
                status,
                code,
                message,
            } => write!(f, "rollover: {} ({}): {}", code, status, message),
            RolloverError::Http(err) => write!(f, "rollover: http error: {}", err),
            RolloverError::Config(msg) => write!(f, "rollover: config error: {}", msg),
        }
    }
}

impl std::error::Error for RolloverError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RolloverError::Http(err) => Some(err),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for RolloverError {
    fn from(err: reqwest::Error) -> Self {
        RolloverError::Http(err)
    }
}

/// Checks whether an error is a Rollover API error with the given error code.
pub fn is_error_code(err: &RolloverError, code: &str) -> bool {
    err.is_code(code)
}

#[derive(serde::Deserialize)]
struct ApiErrorBody {
    #[serde(default)]
    code: String,
    #[serde(default)]
    message: String,
}

pub(crate) fn parse_error(status: u16, body: &[u8]) -> RolloverError {
    if body.is_empty() {
        return RolloverError::Api {
            status,
            code: "http_error".to_string(),
            message: default_status_text(status),
        };
    }

    match serde_json::from_slice::<ApiErrorBody>(body) {
        Ok(parsed) => RolloverError::Api {
            status,
            code: if parsed.code.is_empty() {
                "unknown_error".to_string()
            } else {
                parsed.code
            },
            message: if parsed.message.is_empty() {
                format!(
                    "unexpected response (HTTP {}): {}",
                    status,
                    String::from_utf8_lossy(body)
                )
            } else {
                parsed.message
            },
        },
        Err(_) => RolloverError::Api {
            status,
            code: "unknown_error".to_string(),
            message: format!(
                "unexpected response (HTTP {}): {}",
                status,
                String::from_utf8_lossy(body)
            ),
        },
    }
}

fn default_status_text(status: u16) -> String {
    match status {
        400 => "Bad Request".to_string(),
        401 => "Unauthorized".to_string(),
        403 => "Forbidden".to_string(),
        404 => "Not Found".to_string(),
        429 => "Too Many Requests".to_string(),
        500 => "Internal Server Error".to_string(),
        502 => "Bad Gateway".to_string(),
        503 => "Service Unavailable".to_string(),
        _ => format!("HTTP {}", status),
    }
}
