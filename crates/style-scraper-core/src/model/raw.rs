use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RawFacts {
    pub schema_version: String,
    pub url: String,
    pub viewport: ViewportFacts,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color_scheme: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub captured_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser: Option<BrowserFacts>,
    pub pages: Vec<PageFacts>,
    #[serde(default)]
    pub diagnostics: Vec<ProbeDiagnostic>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ViewportFacts {
    pub width: u32,
    pub height: u32,
    #[serde(default = "default_device_scale_factor")]
    pub device_scale_factor: f64,
}

fn default_device_scale_factor() -> f64 {
    1.0
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserFacts {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_agent_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PageFacts {
    pub url: String,
    #[serde(default)]
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewport: Option<ViewportFacts>,
    #[serde(default)]
    pub dom: DomFacts,
    #[serde(default)]
    pub cssom: CssomFacts,
    #[serde(default)]
    pub layout: LayoutFacts,
    #[serde(default)]
    pub accessibility: AccessibilityFacts,
    #[serde(default)]
    pub screenshots: Vec<ScreenshotFacts>,
    #[serde(default)]
    pub pseudo_elements: Vec<RawPseudoElement>,
    #[serde(default)]
    pub assets: Vec<RawAsset>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stylesheet_provenance: Option<Value>,
    #[serde(default)]
    pub state_deltas: Vec<RawStateDelta>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct DomFacts {
    #[serde(default)]
    pub nodes: Vec<RawDomNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RawDomNode {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub tag: String,
    #[serde(default)]
    pub attributes: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    #[serde(default)]
    pub text_length: usize,
    #[serde(default)]
    pub visible: bool,
    #[serde(default)]
    pub child_index: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct CssomFacts {
    #[serde(default)]
    pub computed_styles: Vec<RawComputedStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RawComputedStyle {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mask_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font_size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_height: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub letter_spacing: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub padding: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gap: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub row_gap: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column_gap: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub border_radius: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub border_width: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub border_style: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub border_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub box_shadow: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opacity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z_index: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transition_duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transition_timing_function: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub animation_duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform: Option<String>,
    #[serde(default)]
    pub css_variables: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct LayoutFacts {
    #[serde(default)]
    pub boxes: Vec<RawLayoutBox>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RawStateDelta {
    pub state: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_signature: Option<String>,
    #[serde(default)]
    pub changed_properties: BTreeMap<String, StatePropertyDelta>,
    #[serde(default)]
    pub safe_interaction: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StatePropertyDelta {
    pub before: String,
    pub after: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RawLayoutBox {
    pub id: String,
    pub node_id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z_index: Option<i32>,
    #[serde(default)]
    pub visible: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct AccessibilityFacts {
    #[serde(default)]
    pub nodes: Vec<RawAccessibilityNode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tree: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RawAccessibilityNode {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    pub role: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub focused: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expanded: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heading_level: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ScreenshotFacts {
    pub id: String,
    #[serde(rename = "type", alias = "screenshot_type")]
    pub screenshot_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    pub sha256: String,
    #[serde(default)]
    pub width: u32,
    #[serde(default)]
    pub height: u32,
    #[serde(default = "default_device_scale_factor")]
    pub device_scale_factor: f64,
    pub viewport: ViewportFacts,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_node_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RawPseudoElement {
    pub node_id: String,
    pub pseudo: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default)]
    pub computed_style: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout_estimate: Option<RawLayoutBox>,
    #[serde(default)]
    pub visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RawAsset {
    pub id: String,
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_node_id: Option<String>,
    pub url_hash: String,
    #[serde(default)]
    pub resolved_url_redacted: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intrinsic_size: Option<RawAssetIntrinsicSize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RawAssetIntrinsicSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProbeDiagnostic {
    pub code: String,
    pub message: String,
    #[serde(default)]
    pub phase: String,
    #[serde(default)]
    pub severity: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recoverable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommended_action: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timings: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_status: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<String>,
}
