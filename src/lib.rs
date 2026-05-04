//! PromptWallet Core Types
//!
//! Shared types for the PromptWallet ecosystem:
//! - OpenAI-compatible chat API types
//! - Artifact storage types
//! - Extension registry types
//!
//! # Usage
//!
//! ```rust
//! use pw_core::chat::{ChatCompletionRequest, Message, Role};
//! use pw_core::artifacts::{Artifact, ArtifactType};
//!
//! let message = Message::user("Hello, world!");
//! ```

pub mod artifacts;
pub mod chat;
pub mod extensions;

// Re-export common types at crate root for convenience
pub use artifacts::{Artifact, ArtifactType, SearchResult};
pub use chat::{
    ChatCompletionRequest, ChatCompletionResponse, Message, MessageContent, PwRagMode, Role,
};
pub use extensions::{ExtensionInfo, ExtensionRegistry, ExtensionStatus, Category, ClientApp};
