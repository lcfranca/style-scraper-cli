use crate::a11y::roles::native_role_for_tag;
use crate::model::morphology::{AtomicUnit, MorphologyEvidence};
use crate::model::raw::{
    RawAccessibilityNode, RawComputedStyle, RawDomNode, RawFacts, RawLayoutBox, RawStateDelta,
};
use crate::model::tokens::DesignTokens;
use crate::normalize::color::normalize_css_color;
use crate::normalize::shadow::normalize_shadow;
use crate::normalize::spacing::spacing_values;
use std::collections::BTreeMap;

pub fn infer_atoms(raw: &RawFacts, tokens: &DesignTokens) -> Vec<AtomicUnit> {
    let mut atoms = Vec::new();
    for page in &raw.pages {
        let a11y_by_node = accessibility_by_node(&page.accessibility.nodes);
        let styles_by_node = styles_by_node(&page.cssom.computed_styles);
        let boxes_by_node = boxes_by_node(&page.layout.boxes);
        let child_counts = child_counts(&page.dom.nodes);
        let state_deltas_by_node = state_deltas_by_node(&page.state_deltas);

        for node in &page.dom.nodes {
            if !node.visible {
                continue;
            }
            let a11y = a11y_by_node.get(&node.id).copied();
            let style = styles_by_node.get(&node.id).copied();
            let layout_box = boxes_by_node.get(&node.id).copied();
            let visible_child_count = child_counts.get(&node.id).copied().unwrap_or_default();
            let Some(kind) = classify_atom(node, a11y, style, layout_box, visible_child_count)
            else {
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
                layout_box_ids: layout_box
                    .map(|layout_box| vec![layout_box.id.clone()])
                    .unwrap_or_default(),
                computed_style_ids: style
                    .map(|style| {
                        vec![style
                            .id
                            .clone()
                            .unwrap_or_else(|| format!("style:{}", style.node_id))]
                    })
                    .unwrap_or_default(),
                gestalt: None,
                pseudo_element_ids: page
                    .pseudo_elements
                    .iter()
                    .filter(|pseudo| pseudo.node_id == node.id)
                    .map(|pseudo| format!("pseudo:{}:{}", pseudo.node_id, pseudo.pseudo))
                    .collect(),
                asset_ids: page
                    .assets
                    .iter()
                    .filter(|asset| asset.source_node_id.as_deref() == Some(node.id.as_str()))
                    .map(|asset| asset.id.clone())
                    .collect(),
            };
            let applied_tokens = style
                .map(|style| bind_tokens_for_style(style, tokens))
                .unwrap_or_default();
            let states = bind_state_tokens(
                state_deltas_by_node
                    .get(&node.id)
                    .map(Vec::as_slice)
                    .unwrap_or(&[]),
                tokens,
            );

            atoms.push(AtomicUnit {
                unit_type: "atom".to_string(),
                id: format!("atom.{}.{index}", kind.replace('-', "_")),
                kind,
                signature,
                evidence,
                tokens: applied_tokens,
                states,
                confidence,
            });
        }
    }
    atoms
}

fn classify_atom(
    node: &RawDomNode,
    a11y: Option<&RawAccessibilityNode>,
    style: Option<&RawComputedStyle>,
    layout_box: Option<&RawLayoutBox>,
    visible_child_count: usize,
) -> Option<String> {
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
        "form" => Some("form-container".to_string()),
        "header" => Some("header-container".to_string()),
        "footer" => Some("footer-container".to_string()),
        "nav" => Some("navigation-container".to_string()),
        "main" => Some("main-container".to_string()),
        "section" | "article" => Some("section-container".to_string()),
        "label" => Some("label".to_string()),
        "img" | "picture" | "svg" => Some("image".to_string()),
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => Some("heading".to_string()),
        "html" | "body" => Some("page-container".to_string()),
        _ if is_visual_surface(style, layout_box) => Some(visual_surface_kind(style)),
        _ if visible_child_count > 0 => Some("container".to_string()),
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
    } else if matches!(node.tag.as_str(), "form" | "main" | "section" | "article") {
        0.68
    } else {
        0.56
    }
}

fn bind_tokens_for_style(
    style: &RawComputedStyle,
    tokens: &DesignTokens,
) -> BTreeMap<String, String> {
    let mut bound = BTreeMap::new();
    if let Some(value) = style.color.as_deref().and_then(normalize_css_color) {
        bind_or_inline(&mut bound, "text_color", &tokens.color, "color", &value);
    }
    if let Some(value) = style
        .background_color
        .as_deref()
        .and_then(normalize_css_color)
    {
        bind_or_inline(
            &mut bound,
            "background_color",
            &tokens.color,
            "color",
            &value,
        );
    }
    if let Some(value) = style.border_color.as_deref().and_then(normalize_css_color) {
        bind_or_inline(&mut bound, "border_color", &tokens.color, "color", &value);
    }
    if let Some(value) = style.border_radius.as_deref().and_then(first_spacing_value) {
        bind_or_inline(
            &mut bound,
            "border_radius",
            &tokens.radius,
            "radius",
            &value,
        );
    }
    if let Some(value) = style.border_width.as_deref().and_then(first_spacing_value) {
        bind_or_inline(&mut bound, "border_width", &tokens.border, "border", &value);
    }
    if let Some(value) = style.padding.as_deref() {
        let values = spacing_values(value);
        if let Some(x) = horizontal_box_value(&values) {
            bind_or_inline(&mut bound, "padding_x", &tokens.spacing, "spacing", x);
        }
        if let Some(y) = vertical_box_value(&values) {
            bind_or_inline(&mut bound, "padding_y", &tokens.spacing, "spacing", y);
        }
    }
    if let Some(value) = style.gap.as_deref().and_then(first_spacing_value) {
        bind_or_inline(&mut bound, "gap", &tokens.spacing, "spacing", &value);
    }
    if let Some(value) = style.box_shadow.as_deref().and_then(normalize_shadow) {
        bind_or_inline(&mut bound, "shadow", &tokens.shadow, "shadow", &value);
    }
    if let Some(value) = style.opacity.as_deref() {
        if value.trim() != "1" {
            bind_or_inline(
                &mut bound,
                "opacity",
                &tokens.opacity,
                "opacity",
                value.trim(),
            );
        }
    }
    if let Some(value) = style.z_index.as_deref() {
        if value.trim() != "auto" {
            bind_or_inline(
                &mut bound,
                "z_index",
                &tokens.z_index,
                "zIndex",
                value.trim(),
            );
        }
    }
    if style.font_size.is_some() || style.font_family.is_some() || style.font_weight.is_some() {
        if let Some(reference) = find_token_reference(&tokens.typography, "typography", None) {
            bound.insert("typography".to_string(), reference);
        } else if let Some(font_size) = style.font_size.as_deref() {
            bound.insert(
                "typography_inline".to_string(),
                font_size.trim().to_string(),
            );
        }
    }
    bound
}

fn bind_state_tokens(
    deltas: &[&RawStateDelta],
    tokens: &DesignTokens,
) -> BTreeMap<String, BTreeMap<String, String>> {
    let mut states = BTreeMap::new();
    for delta in deltas {
        let state_tokens = states
            .entry(delta.state.clone())
            .or_insert_with(BTreeMap::new);
        for (property, change) in &delta.changed_properties {
            let after = change.after.trim();
            match property.as_str() {
                "color" | "background_color" | "border_color" | "outline_color" => {
                    if let Some(color) = normalize_css_color(after) {
                        bind_or_inline(state_tokens, property, &tokens.color, "color", &color);
                    }
                }
                "border_radius" | "outline_width" => {
                    if let Some(value) = first_spacing_value(after) {
                        bind_or_inline(state_tokens, property, &tokens.radius, "radius", &value);
                    }
                }
                "box_shadow" => {
                    if let Some(value) = normalize_shadow(after) {
                        bind_or_inline(state_tokens, property, &tokens.shadow, "shadow", &value);
                    }
                }
                "opacity" => {
                    bind_or_inline(state_tokens, property, &tokens.opacity, "opacity", after);
                }
                _ => {
                    state_tokens.insert(format!("{property}_inline"), after.to_string());
                }
            }
        }
    }
    states
}

fn bind_or_inline(
    output: &mut BTreeMap<String, String>,
    key: &str,
    token_root: &serde_json::Value,
    token_prefix: &str,
    value: &str,
) {
    if let Some(reference) = find_token_reference(token_root, token_prefix, Some(value)) {
        output.insert(key.to_string(), reference);
    } else {
        output.insert(format!("{key}_inline"), value.to_string());
    }
}

fn find_token_reference(
    root: &serde_json::Value,
    prefix: &str,
    expected_value: Option<&str>,
) -> Option<String> {
    fn walk(
        value: &serde_json::Value,
        path: &mut Vec<String>,
        expected_value: Option<&str>,
    ) -> Option<Vec<String>> {
        let object = value.as_object()?;
        if let Some(token_value) = object.get("$value") {
            let matches = expected_value
                .map(|expected| token_value_matches(token_value, expected))
                .unwrap_or(true);
            if matches {
                return Some(path.clone());
            }
        }
        for (key, child) in object {
            if key.starts_with('$') {
                continue;
            }
            path.push(key.clone());
            if let Some(found) = walk(child, path, expected_value) {
                return Some(found);
            }
            path.pop();
        }
        None
    }

    let mut path = Vec::new();
    walk(root, &mut path, expected_value).map(|path| format!("{{{}.{}}}", prefix, path.join(".")))
}

fn token_value_matches(token_value: &serde_json::Value, expected: &str) -> bool {
    match token_value {
        serde_json::Value::String(value) => value == expected,
        serde_json::Value::Number(value) => value.to_string() == expected,
        _ => false,
    }
}

fn first_spacing_value(value: &str) -> Option<String> {
    spacing_values(value).into_iter().next()
}

fn horizontal_box_value(values: &[String]) -> Option<&String> {
    match values.len() {
        0 => None,
        1 => values.first(),
        2 | 3 => values.get(1),
        _ => values.get(1),
    }
}

fn vertical_box_value(values: &[String]) -> Option<&String> {
    values.first()
}

fn is_visual_surface(style: Option<&RawComputedStyle>, layout_box: Option<&RawLayoutBox>) -> bool {
    let Some(style) = style else {
        return false;
    };
    let has_background = style
        .background_color
        .as_deref()
        .and_then(normalize_css_color)
        .is_some();
    let has_border = style
        .border_width
        .as_deref()
        .and_then(first_spacing_value)
        .and_then(|value| value.strip_suffix("px")?.parse::<f64>().ok())
        .is_some_and(|width| width > 0.0);
    let has_radius = style
        .border_radius
        .as_deref()
        .and_then(first_spacing_value)
        .and_then(|value| value.strip_suffix("px")?.parse::<f64>().ok())
        .is_some_and(|radius| radius > 0.0);
    let has_shadow = style
        .box_shadow
        .as_deref()
        .is_some_and(|shadow| shadow.trim() != "none");
    let visible_area = layout_box
        .map(|layout_box| layout_box.width * layout_box.height)
        .unwrap_or_default();
    (has_background || has_border || has_radius || has_shadow) && visible_area >= 64.0
}

fn visual_surface_kind(style: Option<&RawComputedStyle>) -> String {
    let Some(style) = style else {
        return "surface".to_string();
    };
    if style
        .box_shadow
        .as_deref()
        .is_some_and(|shadow| shadow.trim() != "none")
        && style
            .border_radius
            .as_deref()
            .and_then(first_spacing_value)
            .is_some()
    {
        "card-surface".to_string()
    } else if style
        .border_color
        .as_deref()
        .and_then(normalize_css_color)
        .is_some()
    {
        "panel-surface".to_string()
    } else {
        "surface".to_string()
    }
}

fn child_counts(nodes: &[RawDomNode]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for node in nodes {
        if !node.visible {
            continue;
        }
        if let Some(parent_id) = &node.parent_id {
            *counts.entry(parent_id.clone()).or_default() += 1;
        }
    }
    counts
}

fn state_deltas_by_node(deltas: &[RawStateDelta]) -> BTreeMap<String, Vec<&RawStateDelta>> {
    let mut by_node: BTreeMap<String, Vec<&RawStateDelta>> = BTreeMap::new();
    for delta in deltas {
        if let Some(node_id) = &delta.target_node_id {
            by_node.entry(node_id.clone()).or_default().push(delta);
        }
    }
    by_node
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
