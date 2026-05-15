use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct Morphology {
    #[serde(default)]
    pub atoms: Vec<AtomicUnit>,
    #[serde(default)]
    pub molecules: Vec<MorphologyGroup>,
    #[serde(default)]
    pub organisms: Vec<MorphologyGroup>,
    #[serde(default)]
    pub templates: Vec<MorphologyGroup>,
    #[serde(default)]
    pub pages: Vec<MorphologyGroup>,
    #[serde(default)]
    pub gestalt_clusters: Vec<GestaltCluster>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AtomicUnit {
    pub unit_type: String,
    pub id: String,
    pub kind: String,
    pub signature: String,
    pub evidence: MorphologyEvidence,
    #[serde(default)]
    pub tokens: BTreeMap<String, String>,
    #[serde(default)]
    pub states: BTreeMap<String, BTreeMap<String, String>>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MorphologyGroup {
    pub unit_type: String,
    pub id: String,
    pub kind: String,
    #[serde(default)]
    pub children: Vec<String>,
    pub evidence: MorphologyEvidence,
    pub confidence: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct MorphologyEvidence {
    #[serde(default)]
    pub dom_node_ids: Vec<String>,
    #[serde(default)]
    pub aom_roles: Vec<String>,
    #[serde(default)]
    pub layout_box_ids: Vec<String>,
    #[serde(default)]
    pub computed_style_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gestalt: Option<GestaltEvidence>,
    #[serde(default)]
    pub pseudo_element_ids: Vec<String>,
    #[serde(default)]
    pub asset_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GestaltEvidence {
    pub cluster_id: String,
    pub proximity_score: f64,
    pub alignment_score: f64,
    pub common_region_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GestaltCluster {
    pub id: String,
    pub node_ids: Vec<String>,
    pub layout_box_ids: Vec<String>,
    pub proximity_score: f64,
    pub alignment_score: f64,
    pub common_region_score: f64,
}
