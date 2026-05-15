use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Default)]
pub struct ClusterCandidate {
    pub value: String,
    pub frequency: usize,
    pub evidence: BTreeSet<String>,
    pub source: String,
}

pub fn add_candidate(
    clusters: &mut BTreeMap<String, ClusterCandidate>,
    value: String,
    evidence_id: String,
    source: &str,
) {
    let entry = clusters
        .entry(value.clone())
        .or_insert_with(|| ClusterCandidate {
            value,
            frequency: 0,
            evidence: BTreeSet::new(),
            source: source.to_string(),
        });
    entry.frequency += 1;
    entry.evidence.insert(evidence_id);
}
