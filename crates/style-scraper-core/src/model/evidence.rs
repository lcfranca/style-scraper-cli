use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EvidenceRef {
    pub id: String,
    pub source: String,
    pub epistemic_level: EpistemicLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EpistemicLevel {
    Observed,
    Derived,
    Inferred,
    Unsupported,
}
