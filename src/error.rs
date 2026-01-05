use thiserror::Error;

/// Errors that can occur when using the WaitHuman client
#[derive(Error, Debug)]
pub enum WaitHumanError {
    /// Request timed out waiting for an answer
    #[error("Request timed out after {elapsed_seconds:.1} seconds")]
    Timeout { elapsed_seconds: f64 },

    /// Network error occurred during HTTP request
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    /// Failed to create confirmation request
    #[error("Failed to create confirmation: {status_text}")]
    CreateFailed { status_text: String },

    /// Failed to poll for answer
    #[error("Failed to poll for answer: {status_text}")]
    PollFailed { status_text: String },

    /// Received unexpected answer type
    #[error("Unexpected answer type: expected {expected}, got {actual}")]
    UnexpectedAnswerType { expected: String, actual: String },

    /// Invalid selected index in answer
    #[error("Invalid selected index: {index}")]
    InvalidSelectedIndex { index: u32 },

    /// Invalid response from server
    #[error("Invalid response from server: {0}")]
    InvalidResponse(String),
}

/// Result type alias for WaitHuman operations
pub type Result<T> = std::result::Result<T, WaitHumanError>;
