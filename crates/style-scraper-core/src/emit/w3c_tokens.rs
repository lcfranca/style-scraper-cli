use crate::cluster::color_kmeans::{add_candidate, ClusterCandidate};
use crate::cluster::spacing_scale::add_spacing;
use crate::cluster::typography_scale::add_typography_value;
use crate::model::raw::{RawComputedStyle, RawFacts};
use crate::model::tokens::DesignTokens;
use crate::normalize::color::normalize_css_color;
use crate::normalize::shadow::normalize_shadow;
use crate::normalize::spacing::spacing_values;
use crate::normalize::typography::{normalize_font_family, normalize_font_weight};
use crate::normalize::units::{first_px, px_token_value};
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, BTreeSet};

pub fn infer_design_tokens(raw: &RawFacts) -> DesignTokens {
    let mut color_clusters: BTreeMap<String, ClusterCandidate> = BTreeMap::new();
    let mut text_color_counts: BTreeMap<String, ClusterCandidate> = BTreeMap::new();
    let mut background_counts: BTreeMap<String, ClusterCandidate> = BTreeMap::new();
    let mut interactive_background_counts: BTreeMap<String, ClusterCandidate> = BTreeMap::new();
    let mut font_family_clusters: BTreeMap<String, ClusterCandidate> = BTreeMap::new();
    let mut font_size_clusters: BTreeMap<String, ClusterCandidate> = BTreeMap::new();
    let mut font_weight_clusters: BTreeMap<String, ClusterCandidate> = BTreeMap::new();
    let mut line_height_clusters: BTreeMap<String, ClusterCandidate> = BTreeMap::new();
    let mut letter_spacing_clusters: BTreeMap<String, ClusterCandidate> = BTreeMap::new();
    let mut spacing_clusters: BTreeMap<String, ClusterCandidate> = BTreeMap::new();
    let mut radius_clusters: BTreeMap<String, ClusterCandidate> = BTreeMap::new();
    let mut border_width_clusters: BTreeMap<String, ClusterCandidate> = BTreeMap::new();
    let mut shadow_clusters: BTreeMap<String, ClusterCandidate> = BTreeMap::new();
    let mut opacity_clusters: BTreeMap<String, ClusterCandidate> = BTreeMap::new();
    let mut z_index_clusters: BTreeMap<String, ClusterCandidate> = BTreeMap::new();
    let mut motion_duration_clusters: BTreeMap<String, ClusterCandidate> = BTreeMap::new();

    let interactive_nodes = interactive_node_ids(raw);

    for page in &raw.pages {
        for style in &page.cssom.computed_styles {
            let evidence_id = style_evidence_id(style);

            if let Some(color) = style.color.as_deref().and_then(normalize_css_color) {
                add_candidate(
                    &mut color_clusters,
                    color.clone(),
                    evidence_id.clone(),
                    "computed-cssom.color",
                );
                add_candidate(
                    &mut text_color_counts,
                    color,
                    evidence_id.clone(),
                    "computed-cssom.color",
                );
            }
            if let Some(color) = style
                .background_color
                .as_deref()
                .and_then(normalize_css_color)
            {
                add_candidate(
                    &mut color_clusters,
                    color.clone(),
                    evidence_id.clone(),
                    "computed-cssom.background-color",
                );
                add_candidate(
                    &mut background_counts,
                    color.clone(),
                    evidence_id.clone(),
                    "computed-cssom.background-color",
                );
                if interactive_nodes.contains(&style.node_id) {
                    add_candidate(
                        &mut interactive_background_counts,
                        color,
                        evidence_id.clone(),
                        "computed-cssom.interactive-background-color",
                    );
                }
            }
            if let Some(color) = style.border_color.as_deref().and_then(normalize_css_color) {
                add_candidate(
                    &mut color_clusters,
                    color,
                    evidence_id.clone(),
                    "computed-cssom.border-color",
                );
            }

            if let Some(value) = style.font_family.as_deref().and_then(normalize_font_family) {
                add_typography_value(
                    &mut font_family_clusters,
                    value,
                    evidence_id.clone(),
                    "computed-cssom.font-family",
                );
            }
            if let Some(value) = style
                .font_size
                .as_deref()
                .and_then(first_px)
                .map(px_token_value)
            {
                add_typography_value(
                    &mut font_size_clusters,
                    value,
                    evidence_id.clone(),
                    "computed-cssom.font-size",
                );
            }
            if let Some(value) = style.font_weight.as_deref().and_then(normalize_font_weight) {
                add_typography_value(
                    &mut font_weight_clusters,
                    value,
                    evidence_id.clone(),
                    "computed-cssom.font-weight",
                );
            }
            if let Some(value) = style.line_height.as_deref() {
                add_typography_value(
                    &mut line_height_clusters,
                    value.trim().to_string(),
                    evidence_id.clone(),
                    "computed-cssom.line-height",
                );
            }
            if let Some(value) = style.letter_spacing.as_deref() {
                if value.trim() != "normal" {
                    add_typography_value(
                        &mut letter_spacing_clusters,
                        value.trim().to_string(),
                        evidence_id.clone(),
                        "computed-cssom.letter-spacing",
                    );
                }
            }

            for source in [
                style.padding.as_deref(),
                style.margin.as_deref(),
                style.gap.as_deref(),
                style.row_gap.as_deref(),
                style.column_gap.as_deref(),
            ]
            .into_iter()
            .flatten()
            {
                for value in spacing_values(source) {
                    add_spacing(
                        &mut spacing_clusters,
                        value,
                        evidence_id.clone(),
                        "computed-cssom.spacing",
                    );
                }
            }

            if let Some(radius) = style.border_radius.as_deref() {
                for value in spacing_values(radius) {
                    add_spacing(
                        &mut radius_clusters,
                        value,
                        evidence_id.clone(),
                        "computed-cssom.border-radius",
                    );
                }
            }
            if let Some(width) = style.border_width.as_deref() {
                for value in spacing_values(width) {
                    add_spacing(
                        &mut border_width_clusters,
                        value,
                        evidence_id.clone(),
                        "computed-cssom.border-width",
                    );
                }
            }
            if let Some(shadow) = style.box_shadow.as_deref().and_then(normalize_shadow) {
                add_candidate(
                    &mut shadow_clusters,
                    shadow,
                    evidence_id.clone(),
                    "computed-cssom.box-shadow",
                );
            }
            if let Some(opacity) = style.opacity.as_deref() {
                let opacity = opacity.trim();
                if !opacity.is_empty() && opacity != "1" {
                    add_candidate(
                        &mut opacity_clusters,
                        opacity.to_string(),
                        evidence_id.clone(),
                        "computed-cssom.opacity",
                    );
                }
            }
            if let Some(z_index) = style.z_index.as_deref() {
                let z_index = z_index.trim();
                if !z_index.is_empty() && z_index != "auto" {
                    add_candidate(
                        &mut z_index_clusters,
                        z_index.to_string(),
                        evidence_id.clone(),
                        "computed-cssom.z-index",
                    );
                }
            }
            for duration in [
                style.transition_duration.as_deref(),
                style.animation_duration.as_deref(),
            ]
            .into_iter()
            .flatten()
            {
                let duration = duration.trim();
                if !duration.is_empty() && duration != "0s" && duration != "0ms" {
                    add_candidate(
                        &mut motion_duration_clusters,
                        duration.to_string(),
                        evidence_id.clone(),
                        "computed-cssom.motion-duration",
                    );
                }
            }
        }
    }

    let color_index = named_core_index(&color_clusters, "c");
    DesignTokens {
        color: color_tokens(
            raw,
            &color_clusters,
            &color_index,
            &text_color_counts,
            &background_counts,
            &interactive_background_counts,
        ),
        font_family: token_category(&font_family_clusters, "family", "fontFamily"),
        font_size: token_category(&font_size_clusters, "size", "dimension"),
        font_weight: token_category(&font_weight_clusters, "weight", "fontWeight"),
        line_height: token_category(&line_height_clusters, "line", "dimension"),
        letter_spacing: token_category(&letter_spacing_clusters, "tracking", "dimension"),
        typography: typography_composites(
            &font_family_clusters,
            &font_size_clusters,
            &font_weight_clusters,
            &line_height_clusters,
        ),
        spacing: token_category(&spacing_clusters, "space", "dimension"),
        radius: token_category(&radius_clusters, "radius", "dimension"),
        border: border_tokens(&border_width_clusters),
        shadow: token_category(&shadow_clusters, "shadow", "shadow"),
        opacity: token_category(&opacity_clusters, "opacity", "number"),
        z_index: token_category(&z_index_clusters, "layer", "number"),
        motion: motion_tokens(&motion_duration_clusters),
        ..DesignTokens::default()
    }
}

fn style_evidence_id(style: &RawComputedStyle) -> String {
    style
        .id
        .clone()
        .unwrap_or_else(|| format!("style:{}", style.node_id))
}

fn interactive_node_ids(raw: &RawFacts) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    for page in &raw.pages {
        for node in &page.dom.nodes {
            let role = node.attributes.get("role").map(String::as_str);
            if matches!(
                node.tag.as_str(),
                "button" | "a" | "input" | "select" | "textarea"
            ) || matches!(role, Some("button" | "link" | "tab" | "menuitem"))
            {
                ids.insert(node.id.clone());
            }
        }
        for node in &page.accessibility.nodes {
            if matches!(
                node.role.as_str(),
                "button" | "link" | "tab" | "menuitem" | "checkbox" | "radio" | "textbox"
            ) {
                if let Some(node_id) = &node.node_id {
                    ids.insert(node_id.clone());
                }
            }
        }
    }
    ids
}

fn named_core_index(
    clusters: &BTreeMap<String, ClusterCandidate>,
    prefix: &str,
) -> BTreeMap<String, String> {
    clusters
        .keys()
        .enumerate()
        .map(|(index, value)| (value.clone(), format!("{prefix}{:03}", index + 1)))
        .collect()
}

fn color_tokens(
    raw: &RawFacts,
    clusters: &BTreeMap<String, ClusterCandidate>,
    index: &BTreeMap<String, String>,
    text_counts: &BTreeMap<String, ClusterCandidate>,
    background_counts: &BTreeMap<String, ClusterCandidate>,
    interactive_background_counts: &BTreeMap<String, ClusterCandidate>,
) -> Value {
    let mut root = Map::new();
    root.insert("core".to_string(), core_group(clusters, index, "color"));

    let mut semantic = Map::new();
    if let Some(candidate) = most_frequent(text_counts) {
        if let Some(name) = index.get(&candidate.value) {
            semantic.insert(
                "text".to_string(),
                nested_single_alias(
                    "default",
                    "color",
                    &format!("{{color.core.{name}}}"),
                    candidate,
                    "dominant observed text color",
                    0.74,
                ),
            );
        }
    }
    if let Some(candidate) = dominant_surface_candidate(raw, background_counts) {
        if let Some(name) = index.get(&candidate.value) {
            semantic.insert(
                "surface".to_string(),
                nested_single_alias(
                    "default",
                    "color",
                    &format!("{{color.core.{name}}}"),
                    candidate,
                    "dominant observed background color",
                    0.62,
                ),
            );
        }
    }
    if let Some(candidate) = most_frequent(interactive_background_counts) {
        if let Some(name) = index.get(&candidate.value) {
            semantic.insert(
                "interactive".to_string(),
                nested_single_alias(
                    "1",
                    "color",
                    &format!("{{color.core.{name}}}"),
                    candidate,
                    "recurrent background color on interactive elements",
                    0.7,
                ),
            );
        }
    }
    root.insert("semantic".to_string(), Value::Object(semantic));
    Value::Object(root)
}

fn dominant_surface_candidate<'a>(
    raw: &RawFacts,
    background_counts: &'a BTreeMap<String, ClusterCandidate>,
) -> Option<&'a ClusterCandidate> {
    let mut area_by_color: BTreeMap<String, f64> = BTreeMap::new();
    for page in &raw.pages {
        let node_by_id: BTreeMap<_, _> = page
            .dom
            .nodes
            .iter()
            .map(|node| (node.id.as_str(), node))
            .collect();
        let box_by_node: BTreeMap<_, _> = page
            .layout
            .boxes
            .iter()
            .map(|layout_box| (layout_box.node_id.as_str(), layout_box))
            .collect();

        for style in &page.cssom.computed_styles {
            let Some(color) = style
                .background_color
                .as_deref()
                .and_then(normalize_css_color)
            else {
                continue;
            };
            let node_weight = node_by_id
                .get(style.node_id.as_str())
                .map(|node| match node.tag.as_str() {
                    "html" | "body" => 4.0,
                    "main" | "section" | "article" => 2.0,
                    _ => 1.0,
                })
                .unwrap_or(1.0);
            let area = box_by_node
                .get(style.node_id.as_str())
                .map(|layout_box| layout_box.width.max(0.0) * layout_box.height.max(0.0))
                .unwrap_or(1.0);
            *area_by_color.entry(color).or_default() += area * node_weight;
        }
    }

    let best_color = area_by_color.into_iter().max_by(|left, right| {
        left.1
            .total_cmp(&right.1)
            .then_with(|| right.0.cmp(&left.0))
    })?;
    background_counts
        .get(&best_color.0)
        .or_else(|| most_frequent(background_counts))
}

fn core_group(
    clusters: &BTreeMap<String, ClusterCandidate>,
    index: &BTreeMap<String, String>,
    token_type: &str,
) -> Value {
    let mut core = Map::new();
    for candidate in clusters.values() {
        if let Some(name) = index.get(&candidate.value) {
            core.insert(
                name.clone(),
                token_value(
                    token_type,
                    &candidate.value,
                    "Observed core token candidate",
                    candidate,
                    0.9,
                ),
            );
        }
    }
    Value::Object(core)
}

fn token_category(
    clusters: &BTreeMap<String, ClusterCandidate>,
    prefix: &str,
    token_type: &str,
) -> Value {
    let index = named_core_index(clusters, prefix);
    let mut root = Map::new();
    root.insert("core".to_string(), core_group(clusters, &index, token_type));
    Value::Object(root)
}

fn border_tokens(border_widths: &BTreeMap<String, ClusterCandidate>) -> Value {
    let mut root = Map::new();
    root.insert(
        "width".to_string(),
        token_category(border_widths, "width", "dimension"),
    );
    Value::Object(root)
}

fn motion_tokens(durations: &BTreeMap<String, ClusterCandidate>) -> Value {
    let mut root = Map::new();
    root.insert(
        "duration".to_string(),
        token_category(durations, "duration", "duration"),
    );
    Value::Object(root)
}

fn typography_composites(
    families: &BTreeMap<String, ClusterCandidate>,
    sizes: &BTreeMap<String, ClusterCandidate>,
    weights: &BTreeMap<String, ClusterCandidate>,
    line_heights: &BTreeMap<String, ClusterCandidate>,
) -> Value {
    let mut root = Map::new();
    let Some(size) = most_frequent(sizes) else {
        return Value::Object(root);
    };
    let mut evidence = size.evidence.clone();
    let family = most_frequent(families);
    let weight = most_frequent(weights);
    let line_height = most_frequent(line_heights);
    if let Some(candidate) = family {
        evidence.extend(candidate.evidence.iter().cloned());
    }
    if let Some(candidate) = weight {
        evidence.extend(candidate.evidence.iter().cloned());
    }
    if let Some(candidate) = line_height {
        evidence.extend(candidate.evidence.iter().cloned());
    }

    root.insert(
        "body".to_string(),
        json!({
            "$type": "typography",
            "$value": {
                "fontFamily": family.map(|candidate| candidate.value.clone()).unwrap_or_else(|| "system-ui".to_string()),
                "fontSize": size.value,
                "fontWeight": weight.map(|candidate| candidate.value.clone()).unwrap_or_else(|| "400".to_string()),
                "lineHeight": line_height.map(|candidate| candidate.value.clone()).unwrap_or_else(|| "normal".to_string())
            },
            "$extensions": {
                "styleScraper": {
                    "evidence": evidence.into_iter().collect::<Vec<_>>(),
                    "confidence": 0.66,
                    "source": "computed-cssom.typography-composite",
                    "epistemic_level": "inferred"
                }
            }
        }),
    );
    Value::Object(root)
}

fn nested_single_alias(
    key: &str,
    token_type: &str,
    value: &str,
    candidate: &ClusterCandidate,
    description: &str,
    confidence: f64,
) -> Value {
    let mut nested = Map::new();
    nested.insert(
        key.to_string(),
        token_value(token_type, value, description, candidate, confidence),
    );
    Value::Object(nested)
}

fn token_value(
    token_type: &str,
    value: &str,
    description: &str,
    candidate: &ClusterCandidate,
    confidence: f64,
) -> Value {
    json!({
        "$type": token_type,
        "$value": value,
        "$description": description,
        "$extensions": {
            "styleScraper": {
                "evidence": candidate.evidence.iter().cloned().collect::<Vec<_>>(),
                "frequency": candidate.frequency,
                "confidence": confidence,
                "source": candidate.source,
                "epistemic_level": if confidence >= 0.9 { "derived" } else { "inferred" }
            }
        }
    })
}

fn most_frequent(clusters: &BTreeMap<String, ClusterCandidate>) -> Option<&ClusterCandidate> {
    clusters.values().max_by(|left, right| {
        left.frequency
            .cmp(&right.frequency)
            .then_with(|| right.value.cmp(&left.value))
    })
}
