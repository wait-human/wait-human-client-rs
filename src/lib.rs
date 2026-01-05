//! # WaitHuman Rust Client
//!
//! A Rust client library for interacting with the WaitHuman API.
//!
//! WaitHuman enables applications to pause execution and request human input
//! or confirmation on demand. This client provides a simple, ergonomic API
//! for creating confirmation requests and waiting for human responses.
//!
//! ## Features
//!
//! - **Async API**: Built on tokio for efficient async I/O
//! - **Type-safe**: Leverages Rust's type system for compile-time safety
//! - **Multiple answer formats**: Support for free text and multiple choice questions
//! - **Configurable timeouts**: Optional timeout support for all requests
//!
//! ## Example
//!
//! ```no_run
//! use wait_human::WaitHuman;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a new client
//!     let client = WaitHuman::new_from_key("your-api-key")?;
//!
//!     // Ask a free-text question
//!     let answer = client.ask_free_text(
//!         "What is your name?",
//!         None::<&str>,
//!         None,
//!     ).await?;
//!
//!     println!("Answer: {}", answer);
//!
//!     // Ask a multiple-choice question
//!     let choice = client.ask_multiple_choice(
//!         "Select an option",
//!         ["Option 1", "Option 2"],
//!         None::<&str>,
//!         None,
//!     ).await?;
//!
//!     println!("Selected: {}", choice);
//!
//!     Ok(())
//! }
//! ```

mod client;
mod error;
mod shared_types;
mod types;

// Public exports
pub use client::WaitHuman;
pub use error::{Result, WaitHumanError};
pub use types::{
    AnswerContent, AnswerFormat, AskOptions, ConfirmationAnswer, ConfirmationAnswerWithDate,
    ConfirmationQuestion, QuestionMethod, WaitHumanConfig,
};
