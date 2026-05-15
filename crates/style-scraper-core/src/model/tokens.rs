use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DesignTokens {
    #[serde(rename = "$description")]
    pub description: String,
    #[serde(default)]
    pub color: Value,
    #[serde(default)]
    pub typography: Value,
    #[serde(default, rename = "fontFamily")]
    pub font_family: Value,
    #[serde(default, rename = "fontSize")]
    pub font_size: Value,
    #[serde(default, rename = "fontWeight")]
    pub font_weight: Value,
    #[serde(default, rename = "lineHeight")]
    pub line_height: Value,
    #[serde(default, rename = "letterSpacing")]
    pub letter_spacing: Value,
    #[serde(default)]
    pub spacing: Value,
    #[serde(default)]
    pub sizing: Value,
    #[serde(default)]
    pub radius: Value,
    #[serde(default)]
    pub border: Value,
    #[serde(default)]
    pub shadow: Value,
    #[serde(default)]
    pub opacity: Value,
    #[serde(default, rename = "zIndex")]
    pub z_index: Value,
    #[serde(default)]
    pub motion: Value,
    #[serde(default)]
    pub breakpoint: Value,
    #[serde(default)]
    pub asset: Value,
}

impl Default for DesignTokens {
    fn default() -> Self {
        let empty = Value::Object(Default::default());
        Self {
            description: "W3C-compatible token dictionary inferred from computed browser facts"
                .to_string(),
            color: empty.clone(),
            typography: empty.clone(),
            font_family: empty.clone(),
            font_size: empty.clone(),
            font_weight: empty.clone(),
            line_height: empty.clone(),
            letter_spacing: empty.clone(),
            spacing: empty.clone(),
            sizing: empty.clone(),
            radius: empty.clone(),
            border: empty.clone(),
            shadow: empty.clone(),
            opacity: empty.clone(),
            z_index: empty.clone(),
            motion: empty.clone(),
            breakpoint: empty.clone(),
            asset: empty,
        }
    }
}
