//! OpenAI-compatible Chat API Types
//!
//! These types match the OpenAI API schema for SDK compatibility.
//! Works with OpenAI, Anthropic (via adapter), Groq, OpenRouter, etc.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// Request Types
// ============================================================================

/// Chat completion request - matches OpenAI schema exactly
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatCompletionRequest {
    /// Model to use (e.g., "gpt-4", "llama-3.1-70b")
    pub model: String,

    /// Messages in the conversation
    pub messages: Vec<Message>,

    /// Sampling temperature (0-2)
    #[serde(default = "default_temperature")]
    pub temperature: f32,

    /// Top-p nucleus sampling
    #[serde(default = "default_top_p")]
    pub top_p: f32,

    /// Number of completions to generate
    #[serde(default = "default_n")]
    pub n: u32,

    /// Whether to stream responses
    #[serde(default)]
    pub stream: bool,

    /// Stop sequences
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,

    /// Maximum tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    /// Presence penalty (-2 to 2)
    #[serde(default)]
    pub presence_penalty: f32,

    /// Frequency penalty (-2 to 2)
    #[serde(default)]
    pub frequency_penalty: f32,

    /// User identifier for abuse detection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    /// Response format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,

    /// Seed for deterministic outputs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,

    /// Tool/function definitions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Tool>,

    /// How to handle tool calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
}

fn default_temperature() -> f32 {
    1.0
}
fn default_top_p() -> f32 {
    1.0
}
fn default_n() -> u32 {
    1
}

impl Default for ChatCompletionRequest {
    fn default() -> Self {
        Self {
            model: "gpt-4".to_string(),
            messages: vec![],
            temperature: default_temperature(),
            top_p: default_top_p(),
            n: default_n(),
            stream: false,
            stop: None,
            max_tokens: None,
            presence_penalty: 0.0,
            frequency_penalty: 0.0,
            user: None,
            response_format: None,
            seed: None,
            tools: vec![],
            tool_choice: None,
        }
    }
}

// ============================================================================
// Message Types
// ============================================================================

/// Message in a chat conversation
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    pub role: Role,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<MessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

impl Message {
    /// Create a system message
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: Role::System,
            content: Some(MessageContent::Text(content.into())),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }

    /// Create a user message
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: Some(MessageContent::Text(content.into())),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }

    /// Create an assistant message
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: Some(MessageContent::Text(content.into())),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }

    /// Create a tool result message
    pub fn tool(tool_call_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: Role::Tool,
            content: Some(MessageContent::Text(content.into())),
            name: None,
            tool_calls: None,
            tool_call_id: Some(tool_call_id.into()),
        }
    }

    /// Get content as text (extracts from Parts if needed)
    pub fn text(&self) -> Option<&str> {
        self.content.as_ref().and_then(|c| c.as_text())
    }
}

/// Role of a message sender
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

/// Message content - can be string or array of content parts
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MessageContent {
    Text(String),
    Parts(Vec<ContentPart>),
}

impl MessageContent {
    /// Get content as text (extracts first text from Parts if needed)
    pub fn as_text(&self) -> Option<&str> {
        match self {
            MessageContent::Text(s) => Some(s),
            MessageContent::Parts(parts) => parts.iter().find_map(|p| {
                if let ContentPart::Text { text } = p {
                    Some(text.as_str())
                } else {
                    None
                }
            }),
        }
    }

    /// Convert to owned String
    pub fn into_text(self) -> Option<String> {
        match self {
            MessageContent::Text(s) => Some(s),
            MessageContent::Parts(parts) => parts.into_iter().find_map(|p| {
                if let ContentPart::Text { text } = p {
                    Some(text)
                } else {
                    None
                }
            }),
        }
    }
}

impl From<String> for MessageContent {
    fn from(s: String) -> Self {
        MessageContent::Text(s)
    }
}

impl From<&str> for MessageContent {
    fn from(s: &str) -> Self {
        MessageContent::Text(s.to_string())
    }
}

/// Content part for multimodal messages
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentPart {
    Text { text: String },
    ImageUrl { image_url: ImageUrl },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImageUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

// ============================================================================
// Tool Types
// ============================================================================

/// Response format specification
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub format_type: String,
}

impl ResponseFormat {
    pub fn json() -> Self {
        Self {
            format_type: "json_object".to_string(),
        }
    }

    pub fn text() -> Self {
        Self {
            format_type: "text".to_string(),
        }
    }
}

/// Tool definition
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tool {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: FunctionDefinition,
}

impl Tool {
    /// Create a function tool
    pub fn function(
        name: impl Into<String>,
        description: Option<String>,
        parameters: Option<serde_json::Value>,
    ) -> Self {
        Self {
            tool_type: "function".to_string(),
            function: FunctionDefinition {
                name: name.into(),
                description,
                parameters,
            },
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FunctionDefinition {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}

/// Tool call in assistant message
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: FunctionCall,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

/// How to select tools
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ToolChoice {
    Mode(String), // "none", "auto", "required"
    Specific {
        #[serde(rename = "type")]
        tool_type: String,
        function: FunctionName,
    },
}

impl ToolChoice {
    pub fn none() -> Self {
        ToolChoice::Mode("none".to_string())
    }

    pub fn auto() -> Self {
        ToolChoice::Mode("auto".to_string())
    }

    pub fn required() -> Self {
        ToolChoice::Mode("required".to_string())
    }

    pub fn function(name: impl Into<String>) -> Self {
        ToolChoice::Specific {
            tool_type: "function".to_string(),
            function: FunctionName { name: name.into() },
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FunctionName {
    pub name: String,
}

// ============================================================================
// Response Types
// ============================================================================

/// Chat completion response - non-streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
}

impl ChatCompletionResponse {
    /// Create a simple response with text content
    pub fn new(model: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id: format!("chatcmpl-{}", Uuid::new_v4()),
            object: "chat.completion".to_string(),
            created: chrono::Utc::now().timestamp(),
            model: model.into(),
            choices: vec![Choice {
                index: 0,
                message: Message::assistant(content),
                finish_reason: Some("stop".to_string()),
                logprobs: None,
            }],
            usage: Usage::default(),
            system_fingerprint: None,
        }
    }

    /// Get the first choice's message content as text
    pub fn text(&self) -> Option<&str> {
        self.choices.first().and_then(|c| c.message.text())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub message: Message,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Streaming chunk response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<ChunkChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
}

impl ChatCompletionChunk {
    pub fn new(
        id: &str,
        model: &str,
        delta: ChunkDelta,
        finish_reason: Option<String>,
    ) -> Self {
        Self {
            id: id.to_string(),
            object: "chat.completion.chunk".to_string(),
            created: chrono::Utc::now().timestamp(),
            model: model.to_string(),
            choices: vec![ChunkChoice {
                index: 0,
                delta,
                finish_reason,
                logprobs: None,
            }],
            system_fingerprint: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkChoice {
    pub index: u32,
    pub delta: ChunkDelta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChunkDelta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCallChunk>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallChunk {
    pub index: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub tool_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<FunctionCallChunk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallChunk {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
}

// ============================================================================
// Models Endpoint
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelsResponse {
    pub object: String,
    pub data: Vec<ModelInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub owned_by: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_window: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u32>,
}

// ============================================================================
// Error Response
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl ErrorResponse {
    pub fn new(message: impl Into<String>, error_type: impl Into<String>) -> Self {
        Self {
            error: ErrorDetail {
                message: message.into(),
                error_type: error_type.into(),
                param: None,
                code: None,
            },
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_constructors() {
        let sys = Message::system("You are helpful");
        assert_eq!(sys.role, Role::System);
        assert_eq!(sys.text(), Some("You are helpful"));

        let user = Message::user("Hello");
        assert_eq!(user.role, Role::User);

        let asst = Message::assistant("Hi there!");
        assert_eq!(asst.role, Role::Assistant);

        let tool = Message::tool("call_123", r#"{"result": 42}"#);
        assert_eq!(tool.role, Role::Tool);
        assert_eq!(tool.tool_call_id, Some("call_123".to_string()));
    }

    #[test]
    fn test_request_serialization() {
        let request = ChatCompletionRequest {
            model: "gpt-4".to_string(),
            messages: vec![Message::user("Hello")],
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["model"], "gpt-4");
        assert_eq!(json["messages"][0]["role"], "user");
    }

    #[test]
    fn test_response_creation() {
        let response = ChatCompletionResponse::new("gpt-4", "Hello!");
        
        assert!(response.id.starts_with("chatcmpl-"));
        assert_eq!(response.object, "chat.completion");
        assert_eq!(response.text(), Some("Hello!"));
    }

    #[test]
    fn test_message_content_variants() {
        let text = MessageContent::Text("hello".to_string());
        assert_eq!(text.as_text(), Some("hello"));

        let parts = MessageContent::Parts(vec![
            ContentPart::Text { text: "world".to_string() },
        ]);
        assert_eq!(parts.as_text(), Some("world"));
    }

    #[test]
    fn test_tool_choice() {
        let auto = ToolChoice::auto();
        let json = serde_json::to_value(&auto).unwrap();
        assert_eq!(json, "auto");

        let specific = ToolChoice::function("get_weather");
        let json = serde_json::to_value(&specific).unwrap();
        assert_eq!(json["function"]["name"], "get_weather");
    }
}
