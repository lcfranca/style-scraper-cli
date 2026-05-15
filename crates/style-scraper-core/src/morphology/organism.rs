use crate::a11y::roles::is_landmark;
use crate::model::morphology::{MorphologyEvidence, MorphologyGroup};
use crate::model::raw::RawFacts;
use std::collections::BTreeSet;

pub fn infer_organisms(raw: &RawFacts) -> Vec<MorphologyGroup> {
    let mut organisms = Vec::new();
    let mut seen_node_ids = BTreeSet::new();
    for page in &raw.pages {
        for a11y in &page.accessibility.nodes {
            if !is_landmark(&a11y.role) {
                continue;
            }
            let Some(node_id) = &a11y.node_id else {
                continue;
            };
            seen_node_ids.insert(node_id.clone());
            let index = organisms.len() + 1;
            organisms.push(MorphologyGroup {
                unit_type: "organism".to_string(),
                id: format!("organism.{}.{index}", a11y.role.replace('-', "_")),
                kind: landmark_kind(&a11y.role).to_string(),
                children: Vec::new(),
                evidence: MorphologyEvidence {
                    dom_node_ids: vec![node_id.clone()],
                    aom_roles: vec![a11y.role.clone()],
                    layout_box_ids: page
                        .layout
                        .boxes
                        .iter()
                        .find(|layout_box| &layout_box.node_id == node_id)
                        .map(|layout_box| vec![layout_box.id.clone()])
                        .unwrap_or_default(),
                    computed_style_ids: page
                        .cssom
                        .computed_styles
                        .iter()
                        .find(|style| &style.node_id == node_id)
                        .map(|style| {
                            vec![style
                                .id
                                .clone()
                                .unwrap_or_else(|| format!("style:{}", style.node_id))]
                        })
                        .unwrap_or_default(),
                    gestalt: None,
                    pseudo_element_ids: Vec::new(),
                    asset_ids: Vec::new(),
                },
                confidence: 0.82,
            });
        }

        for node in page.dom.nodes.iter().filter(|node| node.visible) {
            if seen_node_ids.contains(&node.id) {
                continue;
            }
            let Some(kind) = heuristic_organism_kind(
                node.tag.as_str(),
                node.attributes.get("class").map(String::as_str),
            ) else {
                continue;
            };
            seen_node_ids.insert(node.id.clone());
            let index = organisms.len() + 1;
            organisms.push(MorphologyGroup {
                unit_type: "organism".to_string(),
                id: format!("organism.{}.{index}", kind.replace('-', "_")),
                kind,
                children: Vec::new(),
                evidence: MorphologyEvidence {
                    dom_node_ids: vec![node.id.clone()],
                    aom_roles: Vec::new(),
                    layout_box_ids: page
                        .layout
                        .boxes
                        .iter()
                        .find(|layout_box| layout_box.node_id == node.id)
                        .map(|layout_box| vec![layout_box.id.clone()])
                        .unwrap_or_default(),
                    computed_style_ids: page
                        .cssom
                        .computed_styles
                        .iter()
                        .find(|style| style.node_id == node.id)
                        .map(|style| {
                            vec![style
                                .id
                                .clone()
                                .unwrap_or_else(|| format!("style:{}", style.node_id))]
                        })
                        .unwrap_or_default(),
                    gestalt: None,
                    pseudo_element_ids: Vec::new(),
                    asset_ids: Vec::new(),
                },
                confidence: 0.64,
            });
        }
    }
    organisms
}

pub fn page_groups(raw: &RawFacts) -> Vec<MorphologyGroup> {
    raw.pages
        .iter()
        .enumerate()
        .map(|(index, page)| MorphologyGroup {
            unit_type: "page".to_string(),
            id: format!("page.rendered.{:03}", index + 1),
            kind: "rendered-route".to_string(),
            children: Vec::new(),
            evidence: MorphologyEvidence {
                dom_node_ids: page.dom.nodes.iter().map(|node| node.id.clone()).collect(),
                aom_roles: page
                    .accessibility
                    .nodes
                    .iter()
                    .map(|node| node.role.clone())
                    .collect(),
                layout_box_ids: page
                    .layout
                    .boxes
                    .iter()
                    .map(|box_| box_.id.clone())
                    .collect(),
                computed_style_ids: page
                    .cssom
                    .computed_styles
                    .iter()
                    .map(|style| {
                        style
                            .id
                            .clone()
                            .unwrap_or_else(|| format!("style:{}", style.node_id))
                    })
                    .collect(),
                gestalt: None,
                pseudo_element_ids: Vec::new(),
                asset_ids: Vec::new(),
            },
            confidence: 0.9,
        })
        .collect()
}

fn landmark_kind(role: &str) -> &'static str {
    match role {
        "navigation" => "navigation",
        "banner" => "header",
        "main" => "main-content",
        "contentinfo" => "footer",
        "complementary" => "sidebar",
        "search" => "search",
        _ => "landmark-region",
    }
}

fn heuristic_organism_kind(tag: &str, class: Option<&str>) -> Option<String> {
    let class = class.unwrap_or("").to_ascii_lowercase();
    match tag {
        "main" => Some("main-content".to_string()),
        "header" => Some("header".to_string()),
        "footer" => Some("footer".to_string()),
        "form" => Some("form-region".to_string()),
        "nav" => Some("navigation".to_string()),
        "section" | "article" => Some("section-region".to_string()),
        "div" if class.contains("login") || class.contains("auth") => {
            Some("centered-auth-checkpoint".to_string())
        }
        "div" if class.contains("footer") => Some("footer".to_string()),
        "div" if class.contains("header") => Some("header".to_string()),
        _ => None,
    }
}
