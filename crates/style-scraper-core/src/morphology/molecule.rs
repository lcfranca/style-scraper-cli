use crate::model::morphology::{
    AtomicUnit, GestaltCluster, GestaltEvidence, MorphologyEvidence, MorphologyGroup,
};
use crate::model::raw::{RawDomNode, RawFacts};
use std::collections::{BTreeMap, BTreeSet};

pub fn infer_molecules(
    raw: &RawFacts,
    atoms: &[AtomicUnit],
    clusters: &[GestaltCluster],
) -> Vec<MorphologyGroup> {
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
    let mut seen_keys = BTreeSet::new();
    infer_gestalt_molecules(
        atoms,
        clusters,
        &atom_by_node,
        &mut molecules,
        &mut seen_keys,
    );
    infer_dom_molecules(raw, atoms, &atom_by_node, &mut molecules, &mut seen_keys);
    molecules
}

fn infer_gestalt_molecules(
    atoms: &[AtomicUnit],
    clusters: &[GestaltCluster],
    atom_by_node: &BTreeMap<String, &AtomicUnit>,
    molecules: &mut Vec<MorphologyGroup>,
    seen_keys: &mut BTreeSet<String>,
) {
    for cluster in clusters {
        let children: Vec<String> = cluster
            .node_ids
            .iter()
            .filter_map(|node_id| atom_by_node.get(node_id).map(|atom| atom.id.clone()))
            .collect();
        if !(2..=8).contains(&children.len()) {
            continue;
        }
        let key = children.join("|");
        if !seen_keys.insert(key) {
            continue;
        }
        let roles = roles_for_children(atoms, &children);
        let kind = molecule_kind(atoms, &roles, &children);
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
                pseudo_element_ids: Vec::new(),
                asset_ids: Vec::new(),
            },
            confidence: 0.68,
        });
    }
}

fn infer_dom_molecules(
    raw: &RawFacts,
    atoms: &[AtomicUnit],
    atom_by_node: &BTreeMap<String, &AtomicUnit>,
    molecules: &mut Vec<MorphologyGroup>,
    seen_keys: &mut BTreeSet<String>,
) {
    for page in &raw.pages {
        let nodes_by_id: BTreeMap<String, &RawDomNode> = page
            .dom
            .nodes
            .iter()
            .map(|node| (node.id.clone(), node))
            .collect();
        for node in page.dom.nodes.iter().filter(|node| node.visible) {
            let Some(kind) = dom_molecule_kind(node, &nodes_by_id, atom_by_node) else {
                continue;
            };
            let children = descendant_atom_ids(node, &nodes_by_id, atom_by_node);
            if !(1..=24).contains(&children.len()) {
                continue;
            }
            let key = format!("{}:{}", kind, children.join("|"));
            if !seen_keys.insert(key) {
                continue;
            }
            let roles = roles_for_children(atoms, &children);
            let index = molecules.len() + 1;
            molecules.push(MorphologyGroup {
                unit_type: "molecule".to_string(),
                id: format!("molecule.{}.{index}", kind.replace('-', "_")),
                kind,
                children,
                evidence: MorphologyEvidence {
                    dom_node_ids: vec![node.id.clone()],
                    aom_roles: roles,
                    layout_box_ids: Vec::new(),
                    computed_style_ids: Vec::new(),
                    gestalt: None,
                    pseudo_element_ids: Vec::new(),
                    asset_ids: Vec::new(),
                },
                confidence: 0.74,
            });
        }
    }
}

fn dom_molecule_kind(
    node: &RawDomNode,
    nodes_by_id: &BTreeMap<String, &RawDomNode>,
    atom_by_node: &BTreeMap<String, &AtomicUnit>,
) -> Option<String> {
    let class = node
        .attributes
        .get("class")
        .map(String::as_str)
        .unwrap_or("");
    let role = node.attributes.get("role").map(String::as_str);
    if node.tag == "form"
        || descendants_have_kinds(node, nodes_by_id, atom_by_node, &["control", "button"])
    {
        return Some("form-action".to_string());
    }
    if role == Some("alert") || class_contains(class, &["alert", "notice", "toast", "banner"]) {
        return Some("alert".to_string());
    }
    if node.tag == "nav" {
        return Some("navigation".to_string());
    }
    if node.tag == "footer" || class_contains(class, &["legal", "footer"]) {
        return Some("footer-legal".to_string());
    }
    if class_contains(class, &["card", "panel", "surface"]) {
        return Some("card".to_string());
    }
    if atom_by_node
        .get(&node.id)
        .is_some_and(|atom| atom.kind == "card-surface" || atom.kind == "panel-surface")
    {
        return Some("card".to_string());
    }
    None
}

fn descendants_have_kinds(
    node: &RawDomNode,
    nodes_by_id: &BTreeMap<String, &RawDomNode>,
    atom_by_node: &BTreeMap<String, &AtomicUnit>,
    expected: &[&str],
) -> bool {
    let found = descendant_atom_ids(node, nodes_by_id, atom_by_node)
        .into_iter()
        .filter_map(|atom_id| atom_by_node.values().find(|atom| atom.id == atom_id))
        .filter(|atom| expected.iter().any(|kind| *kind == atom.kind))
        .map(|atom| atom.kind.as_str())
        .collect::<BTreeSet<_>>();
    expected.iter().all(|kind| found.contains(*kind))
}

fn descendant_atom_ids(
    node: &RawDomNode,
    nodes_by_id: &BTreeMap<String, &RawDomNode>,
    atom_by_node: &BTreeMap<String, &AtomicUnit>,
) -> Vec<String> {
    let mut children = atom_by_node
        .iter()
        .filter_map(|(node_id, atom)| {
            if node_id == &node.id || is_descendant_of(node_id, &node.id, nodes_by_id) {
                Some(atom.id.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    children.sort();
    children.dedup();
    children
}

fn is_descendant_of(
    candidate_id: &str,
    ancestor_id: &str,
    nodes_by_id: &BTreeMap<String, &RawDomNode>,
) -> bool {
    let mut current = nodes_by_id
        .get(candidate_id)
        .and_then(|node| node.parent_id.as_deref());
    while let Some(parent_id) = current {
        if parent_id == ancestor_id {
            return true;
        }
        current = nodes_by_id
            .get(parent_id)
            .and_then(|node| node.parent_id.as_deref());
    }
    false
}

fn roles_for_children(atoms: &[AtomicUnit], children: &[String]) -> Vec<String> {
    let mut roles: Vec<String> = children
        .iter()
        .filter_map(|child| atoms.iter().find(|atom| &atom.id == child))
        .flat_map(|atom| atom.evidence.aom_roles.clone())
        .collect();
    roles.sort();
    roles.dedup();
    roles
}

fn molecule_kind(atoms: &[AtomicUnit], roles: &[String], children: &[String]) -> String {
    let child_kinds = children
        .iter()
        .filter_map(|child| atoms.iter().find(|atom| &atom.id == child))
        .map(|atom| atom.kind.as_str())
        .collect::<BTreeSet<_>>();
    if roles.iter().any(|role| role == "textbox") && roles.iter().any(|role| role == "button") {
        "search-or-input-group".to_string()
    } else if child_kinds.contains("button") && child_kinds.len() <= 3 {
        "button-composite".to_string()
    } else if child_kinds.contains("button") || child_kinds.contains("control") {
        "control-group".to_string()
    } else if child_kinds.contains("heading") && child_kinds.contains("image") {
        "alert-or-media-heading".to_string()
    } else {
        "visual-group".to_string()
    }
}

fn class_contains(class: &str, needles: &[&str]) -> bool {
    let class = class.to_ascii_lowercase();
    needles.iter().any(|needle| class.contains(needle))
}
