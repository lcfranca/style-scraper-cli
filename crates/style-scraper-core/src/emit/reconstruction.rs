use crate::model::morphology::Morphology;
use crate::model::output::{
    LayoutConstraint, ReconstructionComponent, ReconstructionModel, ReconstructionPageModel,
    ResponsiveSummary,
};
use crate::model::raw::RawFacts;
use crate::model::tokens::DesignTokens;
use std::collections::BTreeMap;

pub fn build_reconstruction_model(
    raw: &RawFacts,
    _tokens: &DesignTokens,
    morphology: &Morphology,
    constraints: &[LayoutConstraint],
) -> ReconstructionModel {
    let mut constraints_by_node: BTreeMap<String, BTreeMap<String, LayoutConstraint>> =
        BTreeMap::new();
    for constraint in constraints {
        constraints_by_node
            .entry(constraint.target_node_id.clone())
            .or_default()
            .insert(constraint.id.clone(), constraint.clone());
    }

    let components = morphology
        .atoms
        .iter()
        .map(|atom| {
            let node_id = atom
                .evidence
                .dom_node_ids
                .first()
                .cloned()
                .unwrap_or_default();
            let mut a11y = BTreeMap::new();
            if let Some(role) = atom.evidence.aom_roles.first() {
                a11y.insert("role".to_string(), role.clone());
            }
            ReconstructionComponent {
                id: atom.id.replace("atom.", "component."),
                component_identity: format!("component.{}", atom.kind.replace('-', "_")),
                kind: atom.kind.clone(),
                tokens: atom.tokens.clone(),
                states: atom.states.clone(),
                a11y,
                layout_constraints: constraints_by_node.remove(&node_id).unwrap_or_default(),
                evidence: atom.evidence.dom_node_ids.clone(),
                confidence: atom.confidence,
            }
        })
        .collect::<Vec<_>>();

    let page_layout = if constraints
        .iter()
        .any(|constraint| constraint.constraint_type == "centered-fixed-width")
    {
        "centered-composition"
    } else if constraints
        .iter()
        .any(|constraint| constraint.constraint_type == "full-width-section")
    {
        "full-width-page"
    } else {
        "rendered-flow"
    };

    let mut page_constraints = BTreeMap::new();
    for constraint in constraints.iter().take(32) {
        page_constraints.insert(constraint.id.clone(), constraint.clone());
    }

    ReconstructionModel {
        page_model: ReconstructionPageModel {
            layout: page_layout.to_string(),
            constraints: page_constraints,
        },
        components,
        asset_references: raw
            .pages
            .iter()
            .flat_map(|page| page.assets.clone())
            .collect(),
        state_specs: raw
            .pages
            .iter()
            .flat_map(|page| page.state_deltas.clone())
            .collect(),
        confidence: reconstruction_confidence(raw, morphology, constraints),
        evidence: raw
            .pages
            .iter()
            .flat_map(|page| {
                page.screenshots
                    .iter()
                    .map(|screenshot| screenshot.id.clone())
            })
            .collect(),
    }
}

pub fn responsive_summary(raw: &RawFacts) -> ResponsiveSummary {
    let mut viewports = raw
        .pages
        .iter()
        .filter_map(|page| page.viewport.as_ref())
        .map(|viewport| format!("{}x{}", viewport.width, viewport.height))
        .collect::<Vec<_>>();
    if viewports.is_empty() {
        viewports.push(format!("{}x{}", raw.viewport.width, raw.viewport.height));
    }
    viewports.sort();
    viewports.dedup();

    ResponsiveSummary {
        viewports,
        breakpoint_candidates: Vec::new(),
        component_variants: Vec::new(),
    }
}

fn reconstruction_confidence(
    raw: &RawFacts,
    morphology: &Morphology,
    constraints: &[LayoutConstraint],
) -> f64 {
    let has_screenshots = raw.pages.iter().any(|page| !page.screenshots.is_empty());
    let has_assets = raw.pages.iter().any(|page| !page.assets.is_empty());
    let has_states = raw.pages.iter().any(|page| !page.state_deltas.is_empty());
    let has_tokens = morphology.atoms.iter().any(|atom| !atom.tokens.is_empty());
    let has_constraints = !constraints.is_empty();
    let score: f64 = 0.35
        + if has_screenshots { 0.18 } else { 0.0 }
        + if has_tokens { 0.18 } else { 0.0 }
        + if has_constraints { 0.12 } else { 0.0 }
        + if has_assets { 0.08 } else { 0.0 }
        + if has_states { 0.08 } else { 0.0 };
    score.min(0.96)
}
