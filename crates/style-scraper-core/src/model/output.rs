use crate::model::diagnostics::Diagnostics;
use crate::model::morphology::Morphology;
use crate::model::raw::{
    RawAccessibilityNode, RawComputedStyle, RawDomNode, RawFacts, RawLayoutBox, RawStateDelta,
    ScreenshotFacts, ViewportFacts,
};
use crate::model::tokens::DesignTokens;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StyleScraperOutput {
    pub schema_version: String,
    pub tool: ToolInfo,
    pub capture: CaptureSummary,
    pub evidence: EvidenceBundle,
    pub tokens: DesignTokens,
    pub morphology: Morphology,
    pub accessibility: AccessibilitySummary,
    pub diagnostics: Diagnostics,
    pub reproducibility: ReproducibilityManifest,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ToolInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CaptureSummary {
    pub url: String,
    pub scope: String,
    pub detail: String,
    pub viewport: ViewportFacts,
    pub color_scheme: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    pub content_hash: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct EvidenceBundle {
    #[serde(default)]
    pub observed_nodes: Vec<RawDomNode>,
    #[serde(default)]
    pub observed_styles: Vec<RawComputedStyle>,
    #[serde(default)]
    pub observed_layout: Vec<RawLayoutBox>,
    #[serde(default)]
    pub observed_accessibility: Vec<RawAccessibilityNode>,
    #[serde(default)]
    pub screenshots: Vec<ScreenshotFacts>,
    #[serde(default)]
    pub observed_state_deltas: Vec<RawStateDelta>,
}

impl EvidenceBundle {
    pub fn from_raw(raw: &RawFacts) -> Self {
        let mut bundle = Self::default();
        for page in &raw.pages {
            bundle.observed_nodes.extend(page.dom.nodes.clone());
            bundle
                .observed_styles
                .extend(page.cssom.computed_styles.clone());
            bundle.observed_layout.extend(page.layout.boxes.clone());
            bundle
                .observed_accessibility
                .extend(page.accessibility.nodes.clone());
            bundle.screenshots.extend(page.screenshots.clone());
            bundle
                .observed_state_deltas
                .extend(page.state_deltas.clone());
        }
        bundle
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct AccessibilitySummary {
    #[serde(default)]
    pub roles: Vec<AccessibilityRoleSummary>,
    #[serde(default)]
    pub contrast: Vec<serde_json::Value>,
    #[serde(default)]
    pub landmarks: Vec<AccessibilityRoleSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AccessibilityRoleSummary {
    pub role: String,
    pub count: usize,
}

impl AccessibilitySummary {
    pub fn from_raw(raw: &RawFacts) -> Self {
        use std::collections::BTreeMap;

        let mut role_counts: BTreeMap<String, usize> = BTreeMap::new();
        let mut landmark_counts: BTreeMap<String, usize> = BTreeMap::new();
        for page in &raw.pages {
            for node in &page.accessibility.nodes {
                *role_counts.entry(node.role.clone()).or_default() += 1;
                if matches!(
                    node.role.as_str(),
                    "banner"
                        | "navigation"
                        | "main"
                        | "contentinfo"
                        | "complementary"
                        | "region"
                        | "search"
                ) {
                    *landmark_counts.entry(node.role.clone()).or_default() += 1;
                }
            }
        }

        Self {
            roles: role_counts
                .into_iter()
                .map(|(role, count)| AccessibilityRoleSummary { role, count })
                .collect(),
            contrast: Vec::new(),
            landmarks: landmark_counts
                .into_iter()
                .map(|(role, count)| AccessibilityRoleSummary { role, count })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReproducibilityManifest {
    pub style_scraper_version: String,
    pub probe_runtime: String,
    pub browser: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_version: Option<String>,
    pub viewport: String,
    pub device_scale_factor: f64,
    pub color_scheme: String,
    pub input_url: String,
    pub capture_hash: String,
    pub analysis_hash: String,
}
