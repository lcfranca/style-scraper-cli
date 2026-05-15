use crate::model::morphology::{
    AtomicUnit, GestaltCluster, GestaltEvidence, MorphologyEvidence, MorphologyGroup,
};
use std::collections::BTreeMap;

pub fn infer_molecules(atoms: &[AtomicUnit], clusters: &[GestaltCluster]) -> Vec<MorphologyGroup> {
    let atom_by_node: BTreeMap<String, &AtomicUnit> = atoms
        .iter()
        .filter_map(|atom| {
            atom.evidence
                .dom_node_ids
                .first()
                .map(|node| (node.clone(), atom))
        })
        .collect();

    let mut molecules = Vec::new();
    for cluster in clusters {
        let children: Vec<String> = cluster
            .node_ids
            .iter()
            .filter_map(|node_id| atom_by_node.get(node_id).map(|atom| atom.id.clone()))
            .collect();
        if !(2..=8).contains(&children.len()) {
            continue;
        }
        let mut roles: Vec<String> = children
            .iter()
            .filter_map(|child| atoms.iter().find(|atom| &atom.id == child))
            .flat_map(|atom| atom.evidence.aom_roles.clone())
            .collect();
        roles.sort();
        roles.dedup();

        let kind = molecule_kind(&roles, &children);
        let index = molecules.len() + 1;
        molecules.push(MorphologyGroup {
            unit_type: "molecule".to_string(),
            id: format!("molecule.{}.{index}", kind.replace('-', "_")),
            kind,
            children,
            evidence: MorphologyEvidence {
                dom_node_ids: cluster.node_ids.clone(),
                aom_roles: roles,
                layout_box_ids: cluster.layout_box_ids.clone(),
                computed_style_ids: Vec::new(),
                gestalt: Some(GestaltEvidence {
                    cluster_id: cluster.id.clone(),
                    proximity_score: cluster.proximity_score,
                    alignment_score: cluster.alignment_score,
                    common_region_score: cluster.common_region_score,
                }),
            },
            confidence: 0.68,
        });
    }
    molecules
}

fn molecule_kind(roles: &[String], children: &[String]) -> String {
    if roles.iter().any(|role| role == "textbox") && roles.iter().any(|role| role == "button") {
        "search-or-input-group".to_string()
    } else if children.iter().any(|child| child.contains("button")) {
        "control-group".to_string()
    } else {
        "visual-group".to_string()
    }
}
