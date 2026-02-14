//! Artifact Storage Types
//!
//! Types for storing and retrieving LLM artifacts (code, documents, summaries, etc.)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// Artifact Types
// ============================================================================

/// Stored artifact with embedding for semantic search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: Uuid,
    pub user_id: String,
    pub artifact_type: ArtifactType,
    pub title: Option<String>,
    pub content: String,
    pub metadata: serde_json::Value,
    pub chunk_index: i32,
    pub total_chunks: i32,
    pub parent_id: Option<Uuid>,
    pub token_count: i32,
    pub created_at: DateTime<Utc>,
}

impl Artifact {
    /// Create a new artifact
    pub fn new(
        user_id: impl Into<String>,
        artifact_type: ArtifactType,
        content: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id: user_id.into(),
            artifact_type,
            title: None,
            content: content.into(),
            metadata: serde_json::json!({}),
            chunk_index: 0,
            total_chunks: 1,
            parent_id: None,
            token_count: 0,
            created_at: Utc::now(),
        }
    }

    /// Set title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set metadata
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = metadata;
        self
    }
}

/// Type of artifact content
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "artifact_type", rename_all = "snake_case"))]
pub enum ArtifactType {
    /// LLM chat response
    ChatResponse,
    /// Document (markdown, text, etc.)
    Document,
    /// Code snippet
    CodeSnippet,
    /// Summary of other content
    Summary,
    /// Analysis or insight
    Analysis,
    /// Custom/other type
    Custom,
}

impl ArtifactType {
    /// Get all artifact types
    pub fn all() -> &'static [ArtifactType] {
        &[
            ArtifactType::ChatResponse,
            ArtifactType::Document,
            ArtifactType::CodeSnippet,
            ArtifactType::Summary,
            ArtifactType::Analysis,
            ArtifactType::Custom,
        ]
    }

    /// Get type as string
    pub fn as_str(&self) -> &'static str {
        match self {
            ArtifactType::ChatResponse => "chat_response",
            ArtifactType::Document => "document",
            ArtifactType::CodeSnippet => "code_snippet",
            ArtifactType::Summary => "summary",
            ArtifactType::Analysis => "analysis",
            ArtifactType::Custom => "custom",
        }
    }
}

impl std::fmt::Display for ArtifactType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================================
// Search Types
// ============================================================================

/// Search result with similarity score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub artifact: Artifact,
    pub similarity: f32,
    pub highlights: Vec<String>,
}

/// Search query parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    /// Search query text
    pub query: String,
    /// Filter by artifact types
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact_types: Option<Vec<ArtifactType>>,
    /// Minimum similarity threshold (0-1)
    #[serde(default = "default_min_similarity")]
    pub min_similarity: f32,
    /// Maximum results to return
    #[serde(default = "default_limit")]
    pub limit: usize,
    /// Filter by tags in metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

fn default_min_similarity() -> f32 {
    0.5
}

fn default_limit() -> usize {
    10
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            query: String::new(),
            artifact_types: None,
            min_similarity: default_min_similarity(),
            limit: default_limit(),
            tags: None,
        }
    }
}

impl SearchQuery {
    /// Create a new search query
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            ..Default::default()
        }
    }

    /// Filter by artifact types
    pub fn with_types(mut self, types: Vec<ArtifactType>) -> Self {
        self.artifact_types = Some(types);
        self
    }

    /// Set minimum similarity
    pub fn with_min_similarity(mut self, threshold: f32) -> Self {
        self.min_similarity = threshold;
        self
    }

    /// Set result limit
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
}

// ============================================================================
// API Request/Response Types
// ============================================================================

/// Request to store an artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreArtifactRequest {
    /// Type of artifact
    pub artifact_type: ArtifactType,
    /// Content to store
    pub content: String,
    /// Optional title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional metadata
    #[serde(default)]
    pub metadata: serde_json::Value,
}

/// Response after storing an artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreArtifactResponse {
    /// IDs of created artifacts (multiple if chunked)
    pub ids: Vec<Uuid>,
    /// Number of chunks created
    pub chunks: usize,
    /// Total tokens in content
    pub token_count: i32,
}

/// Request to search artifacts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchArtifactsRequest {
    /// Search query
    pub query: String,
    /// Filter by artifact types
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_types: Option<Vec<ArtifactType>>,
    /// Minimum similarity (0-1)
    #[serde(default = "default_min_similarity")]
    pub min_similarity: f32,
    /// Maximum results
    #[serde(default = "default_limit")]
    pub limit: usize,
}

/// Response from artifact search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchArtifactsResponse {
    pub results: Vec<SearchResult>,
    pub total: usize,
    pub query: String,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artifact_creation() {
        let artifact = Artifact::new("user123", ArtifactType::CodeSnippet, "fn main() {}")
            .with_title("Main function")
            .with_metadata(serde_json::json!({"language": "rust"}));

        assert_eq!(artifact.user_id, "user123");
        assert_eq!(artifact.artifact_type, ArtifactType::CodeSnippet);
        assert_eq!(artifact.title, Some("Main function".to_string()));
    }

    #[test]
    fn test_artifact_type_serialization() {
        let types = vec![
            (ArtifactType::ChatResponse, "\"chat_response\""),
            (ArtifactType::Document, "\"document\""),
            (ArtifactType::CodeSnippet, "\"code_snippet\""),
            (ArtifactType::Summary, "\"summary\""),
            (ArtifactType::Analysis, "\"analysis\""),
            (ArtifactType::Custom, "\"custom\""),
        ];

        for (artifact_type, expected) in types {
            let json = serde_json::to_string(&artifact_type).unwrap();
            assert_eq!(json, expected);
        }
    }

    #[test]
    fn test_search_query_builder() {
        let query = SearchQuery::new("rust error handling")
            .with_types(vec![ArtifactType::CodeSnippet, ArtifactType::Document])
            .with_min_similarity(0.7)
            .with_limit(5);

        assert_eq!(query.query, "rust error handling");
        assert_eq!(query.artifact_types.unwrap().len(), 2);
        assert_eq!(query.min_similarity, 0.7);
        assert_eq!(query.limit, 5);
    }

    #[test]
    fn test_search_result_serialization() {
        let result = SearchResult {
            artifact: Artifact::new("user", ArtifactType::Document, "Hello world"),
            similarity: 0.95,
            highlights: vec!["Hello".to_string()],
        };

        let json = serde_json::to_value(&result).unwrap();
        assert!((json["similarity"].as_f64().unwrap() - 0.95).abs() < 0.01);
        assert_eq!(json["highlights"][0], "Hello");
    }

    #[test]
    fn test_store_request_serialization() {
        let request = StoreArtifactRequest {
            artifact_type: ArtifactType::CodeSnippet,
            content: "let x = 42;".to_string(),
            title: Some("Variable".to_string()),
            metadata: serde_json::json!({"language": "rust"}),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["artifact_type"], "code_snippet");
        assert_eq!(json["content"], "let x = 42;");
    }
}
