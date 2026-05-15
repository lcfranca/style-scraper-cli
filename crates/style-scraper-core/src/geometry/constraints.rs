use crate::model::output::LayoutConstraint;
use crate::model::raw::{RawFacts, RawLayoutBox, ViewportFacts};
use std::collections::BTreeMap;

pub fn infer_layout_constraints(raw: &RawFacts) -> Vec<LayoutConstraint> {
    let mut constraints = Vec::new();
    for page in &raw.pages {
        let viewport = page.viewport.as_ref().unwrap_or(&raw.viewport);
        for layout_box in page.layout.boxes.iter().filter(|layout_box| {
            layout_box.visible && layout_box.width > 0.0 && layout_box.height > 0.0
        }) {
            if let Some(constraint) =
                centered_fixed_width(layout_box, viewport, constraints.len() + 1)
            {
                constraints.push(constraint);
                continue;
            }
            if let Some(constraint) =
                full_width_section(layout_box, viewport, constraints.len() + 1)
            {
                constraints.push(constraint);
            }
        }
    }
    constraints
}

fn centered_fixed_width(
    layout_box: &RawLayoutBox,
    viewport: &ViewportFacts,
    index: usize,
) -> Option<LayoutConstraint> {
    let viewport_width = f64::from(viewport.width);
    let expected_x = (viewport_width - layout_box.width) / 2.0;
    let centered = (layout_box.x - expected_x).abs() <= 3.0;
    if !centered || layout_box.width >= viewport_width * 0.9 || layout_box.width < 120.0 {
        return None;
    }

    let mut values = BTreeMap::new();
    values.insert("width".to_string(), format_px(layout_box.width));
    values.insert("margin_inline".to_string(), "auto".to_string());
    Some(LayoutConstraint {
        id: format!("constraint:centered-fixed-width:{index:03}"),
        target_node_id: layout_box.node_id.clone(),
        constraint_type: "centered-fixed-width".to_string(),
        values,
        evidence: vec![layout_box.id.clone()],
        confidence: 0.86,
    })
}

fn full_width_section(
    layout_box: &RawLayoutBox,
    viewport: &ViewportFacts,
    index: usize,
) -> Option<LayoutConstraint> {
    let viewport_width = f64::from(viewport.width);
    if layout_box.x.abs() > 2.0 || layout_box.width < viewport_width * 0.96 {
        return None;
    }

    let mut values = BTreeMap::new();
    values.insert("width".to_string(), "100vw".to_string());
    Some(LayoutConstraint {
        id: format!("constraint:full-width-section:{index:03}"),
        target_node_id: layout_box.node_id.clone(),
        constraint_type: "full-width-section".to_string(),
        values,
        evidence: vec![layout_box.id.clone()],
        confidence: 0.78,
    })
}

fn format_px(value: f64) -> String {
    let rounded = (value * 1000.0).round() / 1000.0;
    if rounded.fract().abs() < f64::EPSILON {
        format!("{}px", rounded as i64)
    } else {
        let formatted = format!("{rounded:.3}");
        format!(
            "{}px",
            formatted.trim_end_matches('0').trim_end_matches('.')
        )
    }
}
