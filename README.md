# pw-core

Shared types for the PromptWallet ecosystem - an OpenAI-compatible AI infrastructure platform.

## Features

- **OpenAI-compatible Chat Types**: Request/response types that work with OpenAI, Anthropic, Groq, OpenRouter, and other LLM providers
- **Artifact Storage Types**: Types for semantic search and artifact management
- **Extension Registry Types**: Types for the PromptWallet extension system

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
pw-core = "0.1"
```

For server-side use with sqlx:

```toml
[dependencies]
pw-core = { version = "0.1", features = ["sqlx"] }
```

## Usage

### Chat Messages

```rust
use pw_core::chat::{ChatCompletionRequest, Message, Role};

// Create messages using convenience methods
let messages = vec![
    Message::system("You are a helpful assistant"),
    Message::user("Hello!"),
];

// Build a request
let request = ChatCompletionRequest {
    model: "gpt-4".to_string(),
    messages,
    ..Default::default()
};
```

### Artifacts

```rust
use pw_core::artifacts::{Artifact, ArtifactType, SearchQuery};

// Create an artifact
let artifact = Artifact::new("user123", ArtifactType::CodeSnippet, "fn main() {}")
    .with_title("Main function")
    .with_metadata(serde_json::json!({"language": "rust"}));

// Build a search query
let query = SearchQuery::new("error handling")
    .with_types(vec![ArtifactType::CodeSnippet])
    .with_limit(10);
```

### Extensions

```rust
use pw_core::extensions::{ExtensionInfo, ExtensionStatus};

// Extension status
let status = ExtensionStatus::Stable;
assert_eq!(status.as_str(), "stable");
```

## Modules

- `pw_core::chat` - OpenAI-compatible chat types (Message, ChatCompletionRequest, ChatCompletionResponse, etc.)
- `pw_core::artifacts` - Artifact storage types (Artifact, ArtifactType, SearchResult, etc.)
- `pw_core::extensions` - Extension registry types (ExtensionInfo, Category, ClientApp, etc.)

## Compatibility

These types are designed to be compatible with:
- OpenAI API
- Anthropic Claude API (via adapter)
- Groq API
- OpenRouter API
- Any OpenAI-compatible provider

## License

MIT
