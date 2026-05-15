pub mod a11y;
pub mod cluster;
pub mod emit;
pub mod geometry;
pub mod model;
pub mod morphology;
pub mod normalize;
pub mod validate;

use anyhow::Context;
use model::diagnostics::Diagnostics;
use model::output::{
    AccessibilitySummary, CaptureSummary, EvidenceBundle, ReproducibilityManifest,
    StyleScraperOutput, ToolInfo,
};
use model::raw::RawFacts;

#[derive(Debug, Clone)]
pub struct AnalysisConfig {
    pub scope: String,
    pub detail: String,
    pub format: String,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            scope: "page".to_string(),
            detail: "all".to_string(),
            format: "json".to_string(),
        }
    }
}

pub fn analyze_raw_facts(
    raw: RawFacts,
    config: AnalysisConfig,
) -> anyhow::Result<StyleScraperOutput> {
    validate::schema::validate_raw_facts(&raw)?;

    let capture_hash = validate::determinism::hash_json(&raw)
        .context("failed to compute deterministic capture hash")?;
    let tokens = emit::w3c_tokens::infer_design_tokens(&raw);
    let gestalt_clusters = geometry::gestalt::cluster_layout(&raw);
    let morphology = morphology::reconcile::infer_morphology(&raw, &tokens, &gestalt_clusters);
    let layout_constraints = geometry::constraints::infer_layout_constraints(&raw);
    let responsive = emit::reconstruction::responsive_summary(&raw);
    let reconstruction = emit::reconstruction::build_reconstruction_model(
        &raw,
        &tokens,
        &morphology,
        &layout_constraints,
    );
    let diagnostics = Diagnostics::from_raw(&raw, &tokens, &morphology);
    let evidence = EvidenceBundle::from_raw(&raw);
    let accessibility = AccessibilitySummary::from_raw(&raw);
    let analysis_hash = validate::determinism::hash_json(&tokens)
        .context("failed to compute deterministic analysis hash")?;

    Ok(StyleScraperOutput {
        schema_version: "style-scraper-output.v1".to_string(),
        tool: ToolInfo {
            name: "style-scraper".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        },
        capture: CaptureSummary {
            url: raw.url.clone(),
            scope: config.scope,
            detail: config.detail,
            viewport: raw.viewport.clone(),
            color_scheme: raw
                .color_scheme
                .clone()
                .unwrap_or_else(|| "light".to_string()),
            timestamp: raw.captured_at.clone(),
            content_hash: format!("sha256:{capture_hash}"),
        },
        evidence,
        tokens,
        morphology,
        layout_constraints,
        responsive,
        reconstruction,
        accessibility,
        diagnostics,
        reproducibility: ReproducibilityManifest {
            style_scraper_version: env!("CARGO_PKG_VERSION").to_string(),
            probe_runtime: raw.runtime.clone().unwrap_or_else(|| "bun".to_string()),
            browser: raw
                .browser
                .as_ref()
                .map(|browser| browser.name.clone())
                .unwrap_or_else(|| "unknown".to_string()),
            browser_version: raw
                .browser
                .as_ref()
                .and_then(|browser| browser.version.clone()),
            viewport: format!("{}x{}", raw.viewport.width, raw.viewport.height),
            device_scale_factor: raw.viewport.device_scale_factor,
            color_scheme: raw.color_scheme.unwrap_or_else(|| "light".to_string()),
            input_url: raw.url,
            capture_hash: format!("sha256:{capture_hash}"),
            analysis_hash: format!("sha256:{analysis_hash}"),
        },
    })
}

pub fn tokens_from_raw_facts(raw: RawFacts) -> anyhow::Result<model::tokens::DesignTokens> {
    validate::schema::validate_raw_facts(&raw)?;
    Ok(emit::w3c_tokens::infer_design_tokens(&raw))
}
