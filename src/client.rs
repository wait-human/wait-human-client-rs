use crate::error::{Result, WaitHumanError};
use crate::types::*;
use reqwest::Client;
use std::time::Instant;
use tokio::time::{sleep, Duration};

const DEFAULT_ENDPOINT: &str = "https://api.waithuman.com";
const POLL_INTERVAL_MS: u64 = 3000;

/// Main WaitHuman client for making requests
#[derive(Debug, Clone)]
pub struct WaitHuman {
    api_key: String,
    endpoint: String,
    client: Client,
}

impl WaitHuman {
    /// Creates a new WaitHuman client from just an API key
    ///
    /// This is a convenience wrapper around `WaitHuman::new()` that uses the default endpoint.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your WaitHuman API key
    ///
    /// # Errors
    ///
    /// Returns an error if the API key is empty
    ///
    /// # Example
    ///
    /// ```no_run
    /// use wait_human::WaitHuman;
    ///
    /// let client = WaitHuman::new_from_key("your-api-key")?;
    /// # Ok::<(), wait_human::WaitHumanError>(())
    /// ```
    pub fn new_from_key<S: Into<String>>(api_key: S) -> Result<Self> {
        Self::new(WaitHumanConfig::new(api_key))
    }

    /// Creates a new WaitHuman client
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration containing API key and optional endpoint
    ///
    /// # Errors
    ///
    /// Returns an error if the API key is empty
    pub fn new(config: WaitHumanConfig) -> Result<Self> {
        if config.api_key.is_empty() {
            return Err(WaitHumanError::InvalidResponse(
                "api_key is mandatory".to_string(),
            ));
        }

        let mut endpoint = config
            .endpoint
            .unwrap_or_else(|| DEFAULT_ENDPOINT.to_string());

        // Remove trailing slash
        if endpoint.ends_with('/') {
            endpoint.pop();
        }

        Ok(Self {
            api_key: config.api_key,
            endpoint,
            client: Client::new(),
        })
    }

    /// General method to ask a question and wait for an answer
    ///
    /// # Arguments
    ///
    /// * `question` - The confirmation question to ask
    /// * `options` - Optional settings like timeout
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The confirmation cannot be created
    /// - Network errors occur
    /// - The request times out
    /// - Polling fails
    pub async fn ask(
        &self,
        question: ConfirmationQuestion,
        options: Option<AskOptions>,
    ) -> Result<ConfirmationAnswerWithDate> {
        let confirmation_id = self.create_confirmation(question).await?;
        let timeout_seconds = options.and_then(|o| o.timeout_seconds);
        self.poll_for_answer(confirmation_id, timeout_seconds).await
    }

    /// Convenience method for free-text questions
    ///
    /// # Arguments
    ///
    /// * `subject` - The question subject/title
    /// * `body` - Optional detailed question body
    /// * `options` - Optional settings like timeout
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The request fails or times out
    /// - The answer type doesn't match (not free text)
    pub async fn ask_free_text<S, B>(
        &self,
        subject: S,
        body: Option<B>,
        options: Option<AskOptions>,
    ) -> Result<String>
    where
        S: Into<String>,
        B: Into<String>,
    {
        let question = ConfirmationQuestion {
            method: QuestionMethod::Push,
            subject: subject.into(),
            body: body.map(|b| b.into()),
            answer_format: AnswerFormat::FreeText,
        };

        let answer = self.ask(question, options).await?;

        match answer.answer.answer_content {
            AnswerContent::FreeText { text } => Ok(text),
            other => Err(WaitHumanError::UnexpectedAnswerType {
                expected: "free_text".to_string(),
                actual: format!("{:?}", other),
            }),
        }
    }

    /// Convenience method for multiple-choice questions (single selection)
    ///
    /// # Arguments
    ///
    /// * `subject` - The question subject/title
    /// * `choices` - Available choices for the user to select from
    /// * `body` - Optional detailed question body
    /// * `options` - Optional settings like timeout
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The request fails or times out
    /// - The answer type doesn't match (not options)
    /// - The selected index is invalid
    pub async fn ask_multiple_choice<S, B, C>(
        &self,
        subject: S,
        choices: C,
        body: Option<B>,
        options: Option<AskOptions>,
    ) -> Result<String>
    where
        S: Into<String>,
        B: Into<String>,
        C: IntoIterator,
        C::Item: Into<String>,
    {
        let choices_vec: Vec<String> = choices.into_iter().map(|c| c.into()).collect();

        let question = ConfirmationQuestion {
            method: QuestionMethod::Push,
            subject: subject.into(),
            body: body.map(|b| b.into()),
            answer_format: AnswerFormat::Options {
                options: choices_vec.clone(),
                multiple: false,
            },
        };

        let answer = self.ask(question, options).await?;

        match answer.answer.answer_content {
            AnswerContent::Options { selected_indexes } => {
                let index = selected_indexes.first().ok_or_else(|| {
                    WaitHumanError::InvalidResponse("No selection received".to_string())
                })?;

                let index_usize = *index as usize;

                choices_vec
                    .get(index_usize)
                    .cloned()
                    .ok_or_else(|| WaitHumanError::InvalidSelectedIndex { index: *index })
            }
            other => Err(WaitHumanError::UnexpectedAnswerType {
                expected: "options".to_string(),
                actual: format!("{:?}", other),
            }),
        }
    }

    // Private helper methods

    async fn create_confirmation(&self, question: ConfirmationQuestion) -> Result<String> {
        let url = format!("{}/confirmations/create", self.endpoint);
        let request_body = CreateConfirmationRequest { question };

        let response = self
            .client
            .post(&url)
            .header("Authorization", &self.api_key)
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(WaitHumanError::CreateFailed {
                status_text: response.status().to_string(),
            });
        }

        let data: CreateConfirmationResponse = response.json().await?;
        Ok(data.confirmation_request_id)
    }

    async fn poll_for_answer(
        &self,
        confirmation_id: String,
        timeout_seconds: Option<u64>,
    ) -> Result<ConfirmationAnswerWithDate> {
        let start = Instant::now();

        loop {
            let elapsed_seconds = start.elapsed().as_secs_f64();

            if let Some(timeout) = timeout_seconds {
                if elapsed_seconds > timeout as f64 {
                    return Err(WaitHumanError::Timeout { elapsed_seconds });
                }
            }

            let url = format!(
                "{}/confirmations/get/{}?long_poll=false",
                self.endpoint, confirmation_id
            );

            let response = self
                .client
                .get(&url)
                .header("Authorization", &self.api_key)
                .send()
                .await?;

            if !response.status().is_success() {
                return Err(WaitHumanError::PollFailed {
                    status_text: response.status().to_string(),
                });
            }

            let data: GetConfirmationResponse = response.json().await?;

            if let Some(answer) = data.maybe_answer {
                return Ok(answer);
            }

            // Wait before next poll
            sleep(Duration::from_millis(POLL_INTERVAL_MS)).await;
        }
    }
}
