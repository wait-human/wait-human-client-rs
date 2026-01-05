# WaitHuman Rust Client

A Rust client library for [WaitHuman](https://waithuman.com) - pause execution and request human input or confirmation on demand.

[![Crates.io](https://img.shields.io/crates/v/wait-human.svg)](https://crates.io/crates/wait-human)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Async API**: Built on tokio for efficient async I/O
- **Type-safe**: Leverages Rust's type system for compile-time safety
- **Multiple answer formats**: Support for free text and multiple choice questions
- **Configurable timeouts**: Optional timeout support for all requests
- **Ergonomic API**: Generic type parameters eliminate the need for `.to_string()` calls

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
wait-human = "0.1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Quick Start

```rust
use wait_human::WaitHuman;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new client
    let client = WaitHuman::new_from_key("your-api-key")?;

    // Ask a multiple choice question
    let answer = client
        .ask_multiple_choice(
            "Send invoice?",
            ["yes, send", "no"],
            Some("Customer asked for a 3-page website. is 500$ ok?"),
            None,
        )
        .await?;

    if answer == "yes, send" {
        println!("Send!");
    } else {
        println!("wait...");
    }

    // Ask a free-text question
    let feedback = client
        .ask_free_text(
            "User Feedback",
            Some("Please explain why you rejected the invoice."),
            None,
        )
        .await?;

    println!("{}", feedback);

    Ok(())
}
```

## API Methods

### `new_from_key(api_key)`

Create a client with just an API key (uses default endpoint):

```rust
let client = WaitHuman::new_from_key("your-api-key")?;
```

### `new(config)`

Create a client with custom configuration:

```rust
use wait_human::WaitHumanConfig;

let client = WaitHuman::new(
    WaitHumanConfig::new("your-api-key")
        .with_endpoint("https://custom.endpoint.com")
)?;
```

### `ask_free_text(subject, body, options)`

Ask an open-ended text question:

```rust
let answer = client
    .ask_free_text(
        "What's your favorite color?",
        Some("Please be specific"),
        None,
    )
    .await?;
```

### `ask_multiple_choice(subject, choices, body, options)`

Ask a multiple-choice question:

```rust
let choice = client
    .ask_multiple_choice(
        "Choose a framework",
        ["Actix", "Rocket", "Axum"],
        None::<&str>,
        None,
    )
    .await?;
```

### `ask(question, options)`

Low-level method for full control:

```rust
use wait_human::{ConfirmationQuestion, QuestionMethod, AnswerFormat};

let question = ConfirmationQuestion {
    method: QuestionMethod::Push,
    subject: "Approve deployment?".to_string(),
    body: Some("Production environment".to_string()),
    answer_format: AnswerFormat::FreeText,
};

let answer = client.ask(question, None).await?;
```

## Timeouts

Configure request timeouts:

```rust
use wait_human::AskOptions;

let answer = client
    .ask_free_text(
        "Quick question",
        None::<&str>,
        Some(AskOptions {
            timeout_seconds: Some(30),
        }),
    )
    .await?;
```

## Error Handling

The library uses `Result<T, WaitHumanError>` for error handling:

```rust
match client.ask_free_text("Question?", None::<&str>, None).await {
    Ok(answer) => println!("Answer: {}", answer),
    Err(e) => eprintln!("Error: {}", e),
}
```

Error types include:

- `Timeout` - Request exceeded timeout
- `NetworkError` - Network connectivity issues
- `CreateFailed` - Failed to create confirmation
- `PollFailed` - Failed to poll for answer
- `UnexpectedAnswerType` - Answer type mismatch
- `InvalidSelectedIndex` - Invalid choice index
- `InvalidResponse` - Unexpected server response

## Examples

Run the demo example:

```bash
cargo run --example demo
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Links

- [Crates.io](https://crates.io/crates/wait-human)
- [Repository](https://github.com/wait-human/wait-human-client-rs)
- [WaitHuman Homepage](https://waithuman.com)
