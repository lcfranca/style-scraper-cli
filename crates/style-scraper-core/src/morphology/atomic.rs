use crate::a11y::roles::native_role_for_tag;
use crate::model::morphology::{AtomicUnit, MorphologyEvidence};
use crate::model::raw::{
    RawAccessibilityNode, RawComputedStyle, RawDomNode, RawFacts, RawLayoutBox,
};
use crate::model::tokens::DesignTokens;
use std::collections::BTreeMap;

pub fn infer_atoms(raw: &RawFacts, _tokens: &DesignTokens) -> Vec<AtomicUnit> {
    let mut atoms = Vec::new();
    for page in &raw.pages {
        let a11y_by_node = accessibility_by_node(&page.accessibility.nodes);
        let styles_by_node = styles_by_node(&page.cssom.computed_styles);
        let boxes_by_node = boxes_by_node(&page.layout.boxes);

        for node in &page.dom.nodes {
            if !node.visible {
                continue;
            }
            let a11y = a11y_by_node.get(&node.id).copied();
            let Some(kind) = classify_atom(node, a11y) else {
                continue;
            };
            let role = a11y
                .map(|node| node.role.clone())
                .or_else(|| native_role_for_tag(&node.tag).map(str::to_string));
            let confidence = atom_confidence(node, role.as_deref(), a11y.is_some());
            let index = atoms.len() + 1;
            let signature = if let Some(role) = &role {
                format!("{}[role={}]", node.tag, role)
            } else {
                node.tag.clone()
            };
            let evidence = MorphologyEvidence {
                dom_node_ids: vec![node.id.clone()],
                aom_roles: role.into_iter().collect(),
                layout_box_ids: boxes_by_node
                    .get(&node.id)
                    .map(|layout_box| vec![layout_box.id.clone()])
                    .unwrap_or_default(),
                computed_style_ids: styles_by_node
                    .get(&node.id)
                    .map(|style| {
                        vec![style
                            .id
                            .clone()
                            .unwrap_or_else(|| format!("style:{}", style.node_id))]
                    })
                    .unwrap_or_default(),
                gestalt: None,
            };
            atoms.push(AtomicUnit {
                unit_type: "atom".to_string(),
                id: format!("atom.{}.{index}", kind.replace('-', "_")),
                kind,
                signature,
                evidence,
                tokens: BTreeMap::new(),
                confidence,
            });
        }
    }
    atoms
}

fn classify_atom(node: &RawDomNode, a11y: Option<&RawAccessibilityNode>) -> Option<String> {
    if let Some(role) = a11y.map(|node| node.role.as_str()) {
        match role {
            "button" => return Some("button".to_string()),
            "link" => return Some("link".to_string()),
            "textbox" | "combobox" | "checkbox" | "radio" | "switch" => {
                return Some("control".to_string())
            }
            "img" => return Some("image".to_string()),
            "heading" => return Some("heading".to_string()),
            "tab" => return Some("tab".to_string()),
            _ => {}
        }
    }

    match node.tag.as_str() {
        "button" => Some("button".to_string()),
        "a" => Some("link".to_string()),
        "input" | "select" | "textarea" => Some("control".to_string()),
        "label" => Some("label".to_string()),
        "img" | "picture" | "svg" => Some("image".to_string()),
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => Some("heading".to_string()),
        _ if node.text_length > 0 => Some("text-run".to_string()),
        _ => None,
    }
}

fn atom_confidence(node: &RawDomNode, role: Option<&str>, has_a11y: bool) -> f64 {
    let native_role = native_role_for_tag(&node.tag);
    if has_a11y && native_role == role {
        0.91
    } else if has_a11y {
        0.78
    } else if native_role.is_some() {
        0.72
    } else {
        0.54
    }
}

fn accessibility_by_node(
    nodes: &[RawAccessibilityNode],
) -> BTreeMap<String, &RawAccessibilityNode> {
    nodes
        .iter()
        .filter_map(|node| node.node_id.as_ref().map(|id| (id.clone(), node)))
        .collect()
}

fn styles_by_node(styles: &[RawComputedStyle]) -> BTreeMap<String, &RawComputedStyle> {
    styles
        .iter()
        .map(|style| (style.node_id.clone(), style))
        .collect()
}

fn boxes_by_node(boxes: &[RawLayoutBox]) -> BTreeMap<String, &RawLayoutBox> {
    boxes
        .iter()
        .map(|layout_box| (layout_box.node_id.clone(), layout_box))
        .collect()
}
