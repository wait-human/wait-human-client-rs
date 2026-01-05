// Re-export shared types from backend
pub use crate::shared_types::{
    AnswerContent, AnswerFormat, ConfirmationAnswer, ConfirmationAnswerWithDate,
    ConfirmationQuestion, QuestionMethod,
};

/// Configuration for the WaitHuman client
#[derive(Debug, Clone)]
pub struct WaitHumanConfig {
    /// Your WaitHuman API key (mandatory)
    pub api_key: String,
    /// Optional custom endpoint URL. Defaults to 'https://api.waithuman.com'
    pub endpoint: Option<String>,
}

impl WaitHumanConfig {
    /// Creates a new WaitHumanConfig with the given API key
    pub fn new<S: Into<String>>(api_key: S) -> Self {
        Self {
            api_key: api_key.into(),
            endpoint: None,
        }
    }

    /// Sets the endpoint URL
    pub fn with_endpoint<S: Into<String>>(mut self, endpoint: S) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
}

/// Options for ask requests
#[derive(Debug, Clone, Default)]
pub struct AskOptions {
    /// Optional timeout in seconds. If None, will poll indefinitely
    pub timeout_seconds: Option<u64>,
}

// Internal API request/response types
#[derive(serde::Serialize, Debug)]
pub(crate) struct CreateConfirmationRequest {
    pub question: ConfirmationQuestion,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct CreateConfirmationResponse {
    pub confirmation_request_id: String,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct GetConfirmationResponse {
    pub maybe_answer: Option<ConfirmationAnswerWithDate>,
}
