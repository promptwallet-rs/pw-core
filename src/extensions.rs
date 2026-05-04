//! Extension Registry Types
//!
//! Types for the PromptWallet extension system.

use serde::{Deserialize, Serialize};

// ============================================================================
// Extension Info
// ============================================================================

/// Extension metadata (from registry)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionInfo {
    pub id: String,
    pub name: String,
    pub tagline: String,
    pub description: String,
    pub icon: String,
    pub category: String,
    pub status: ExtensionStatus,
    pub features: Vec<String>,
    #[serde(rename = "requiredBy")]
    pub required_by: Vec<String>,
    pub docs: String,
    pub pricing: String,
}

/// Extension status
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExtensionStatus {
    Planned,
    Beta,
    Stable,
    Deprecated,
}

impl ExtensionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ExtensionStatus::Planned => "planned",
            ExtensionStatus::Beta => "beta",
            ExtensionStatus::Stable => "stable",
            ExtensionStatus::Deprecated => "deprecated",
        }
    }
}

impl std::fmt::Display for ExtensionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================================
// Category
// ============================================================================

/// Extension category
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
}

// ============================================================================
// Client App
// ============================================================================

/// Client application configuration
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientApp {
    pub id: String,
    pub name: String,
    pub description: String,
    pub requires: Vec<String>,
    pub optional: Vec<String>,
}

// ============================================================================
// Registry
// ============================================================================

/// Full extension registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionRegistry {
    pub version: String,
    pub extensions: Vec<ExtensionInfo>,
    pub categories: Vec<Category>,
    #[serde(rename = "clientApps")]
    pub client_apps: Vec<ClientApp>,
}

impl ExtensionRegistry {
    /// Get extension by ID
    pub fn get_extension(&self, id: &str) -> Option<&ExtensionInfo> {
        self.extensions.iter().find(|e| e.id == id)
    }

    /// Get extensions by category
    pub fn get_by_category(&self, category: &str) -> Vec<&ExtensionInfo> {
        self.extensions
            .iter()
            .filter(|e| e.category == category)
            .collect()
    }

    /// Get extensions by status
    pub fn get_by_status(&self, status: ExtensionStatus) -> Vec<&ExtensionInfo> {
        self.extensions
            .iter()
            .filter(|e| e.status == status)
            .collect()
    }

    /// Get category by ID
    pub fn get_category(&self, id: &str) -> Option<&Category> {
        self.categories.iter().find(|c| c.id == id)
    }

    /// Get client app by ID
    pub fn get_client_app(&self, id: &str) -> Option<&ClientApp> {
        self.client_apps.iter().find(|a| a.id == id)
    }
}

// ============================================================================
// API Response Types
// ============================================================================

/// Response for listing extensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionsListResponse {
    pub extensions: Vec<ExtensionInfo>,
    pub categories: Vec<Category>,
    #[serde(rename = "clientApps")]
    pub client_apps: Vec<ClientApp>,
    pub version: String,
}

/// Response for extension details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionDetailResponse {
    pub extension: ExtensionInfo,
    pub category: Option<Category>,
    pub required_by_apps: Vec<String>,
    pub is_loaded: bool,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_status_serialization() {
        let statuses = vec![
            (ExtensionStatus::Planned, "\"planned\""),
            (ExtensionStatus::Beta, "\"beta\""),
            (ExtensionStatus::Stable, "\"stable\""),
            (ExtensionStatus::Deprecated, "\"deprecated\""),
        ];

        for (status, expected) in statuses {
            let json = serde_json::to_string(&status).unwrap();
            assert_eq!(json, expected);
        }
    }

    #[test]
    fn test_extension_info_serialization() {
        let info = ExtensionInfo {
            id: "pw-workspace".to_string(),
            name: "Workspace".to_string(),
            tagline: "Index code".to_string(),
            description: "Full description".to_string(),
            icon: "folder".to_string(),
            category: "coding".to_string(),
            status: ExtensionStatus::Stable,
            features: vec!["Feature 1".to_string()],
            required_by: vec!["app1".to_string()],
            docs: "/docs/workspace.md".to_string(),
            pricing: "included".to_string(),
        };

        let json = serde_json::to_value(&info).unwrap();
        assert_eq!(json["id"], "pw-workspace");
        assert_eq!(json["status"], "stable");
        assert_eq!(json["requiredBy"][0], "app1");
    }

    #[test]
    fn test_registry_lookup() {
        let registry = ExtensionRegistry {
            version: "1.0".to_string(),
            extensions: vec![
                ExtensionInfo {
                    id: "ext1".to_string(),
                    name: "Extension 1".to_string(),
                    tagline: "...".to_string(),
                    description: "...".to_string(),
                    icon: "icon".to_string(),
                    category: "coding".to_string(),
                    status: ExtensionStatus::Stable,
                    features: vec![],
                    required_by: vec![],
                    docs: "".to_string(),
                    pricing: "free".to_string(),
                },
            ],
            categories: vec![
                Category {
                    id: "coding".to_string(),
                    name: "Coding".to_string(),
                    description: "Dev tools".to_string(),
                    icon: "code".to_string(),
                },
            ],
            client_apps: vec![],
        };

        assert!(registry.get_extension("ext1").is_some());
        assert!(registry.get_extension("nonexistent").is_none());
        assert_eq!(registry.get_by_category("coding").len(), 1);
        assert!(registry.get_category("coding").is_some());
    }
}
