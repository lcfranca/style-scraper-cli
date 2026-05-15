use crate::model::morphology::Morphology;
use crate::model::raw::RawFacts;
use crate::model::tokens::DesignTokens;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct Diagnostics {
    #[serde(default)]
    pub warnings: Vec<DiagnosticMessage>,
    #[serde(default)]
    pub unsupported: Vec<DiagnosticMessage>,
    #[serde(default)]
    pub confidence_summary: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DiagnosticMessage {
    pub code: String,
    pub message: String,
    pub phase: String,
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

impl Diagnostics {
    pub fn from_raw(raw: &RawFacts, _tokens: &DesignTokens, morphology: &Morphology) -> Self {
        let warnings = raw
            .diagnostics
            .iter()
            .map(|diagnostic| DiagnosticMessage {
                code: diagnostic.code.clone(),
                message: diagnostic.message.clone(),
                phase: diagnostic.phase.clone(),
                severity: diagnostic.severity.clone(),
                recoverable: diagnostic.recoverable,
                recommended_action: diagnostic.recommended_action.clone(),
                timings: diagnostic.timings.clone(),
                completed: diagnostic.completed.clone(),
                http_status: diagnostic.http_status,
                retry_after: diagnostic.retry_after.clone(),
            })
            .collect();

        let mut confidence_summary = BTreeMap::new();
        confidence_summary.insert(
            "atoms".to_string(),
            json!({
                "count": morphology.atoms.len(),
                "mean": mean_confidence(morphology.atoms.iter().map(|atom| atom.confidence))
            }),
        );
        confidence_summary.insert(
            "molecules".to_string(),
            json!({
                "count": morphology.molecules.len(),
                "mean": mean_confidence(morphology.molecules.iter().map(|group| group.confidence))
            }),
        );

        Self {
            warnings,
            unsupported: Vec::new(),
            confidence_summary,
        }
    }
}

fn mean_confidence(values: impl Iterator<Item = f64>) -> f64 {
    let mut count = 0.0;
    let mut total = 0.0;
    for value in values {
        count += 1.0;
        total += value;
    }
    if count == 0.0 {
        0.0
    } else {
        (total / count * 1000.0).round() / 1000.0
    }
}
