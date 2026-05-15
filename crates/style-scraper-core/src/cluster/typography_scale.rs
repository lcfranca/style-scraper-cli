use super::color_kmeans::{add_candidate, ClusterCandidate};
use std::collections::BTreeMap;

pub fn add_typography_value(
    clusters: &mut BTreeMap<String, ClusterCandidate>,
    value: String,
    evidence_id: String,
    source: &str,
) {
    add_candidate(clusters, value, evidence_id, source);
}
