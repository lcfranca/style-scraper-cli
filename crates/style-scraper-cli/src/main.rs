mod args;
mod exit_codes;
mod output;

use args::{Cli, Commands};
use clap::Parser;
use serde::Serialize;
use serde_json::json;
use std::fs;
use std::process::ExitCode;
use style_scraper_core::emit::{css_vars, tailwind};
use style_scraper_core::model::raw::RawFacts;
use style_scraper_core::validate::determinism::hex_sha256;
use style_scraper_core::{analyze_raw_facts, tokens_from_raw_facts, AnalysisConfig};
use style_scraper_probe_runner::{BunProbeRunner, CaptureConfig, ProbeRunner, ProbeRunnerError};
use tracing_subscriber::EnvFilter;

fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .without_time()
        .init();

    let cli = Cli::parse();
    match run(cli) {
        Ok(()) => ExitCode::from(exit_codes::SUCCESS as u8),
        Err(error) => {
            match serde_json::to_string(&error.to_stderr_payload()) {
                Ok(payload) => eprintln!("{payload}"),
                Err(write_error) => eprintln!(
                    "{{\"error\":{{\"code\":\"OUTPUT_WRITE_FAILURE\",\"message\":\"{}\"}}}}",
                    write_error
                ),
            }
            ExitCode::from(error.exit_code as u8)
        }
    }
}

fn run(cli: Cli) -> Result<(), CliError> {
    match cli.command {
        Commands::Extract(args) => {
            let config = CaptureConfig {
                url: args.url.clone(),
                viewport: args.viewport,
                viewports: args.viewports,
                color_scheme: args.color_scheme.as_str().to_string(),
                states: args.states,
                include_screenshots: args.include_screenshots,
                screenshot: args.screenshot.as_str().to_string(),
                wait: if args.safe_capture {
                    "auto".to_string()
                } else {
                    args.wait.as_str().to_string()
                },
                wait_for_selector: args.wait_for_selector,
                timeout_ms: args.timeout_ms,
                navigation_timeout_ms: args.navigation_timeout_ms,
                capture_timeout_ms: args.capture_timeout_ms,
                stability_window_ms: args.stability_window_ms,
                max_stability_wait_ms: args.max_stability_wait_ms,
                ignore_networkidle_timeout: args.ignore_networkidle_timeout,
                capture_on_timeout: args.capture_on_timeout || args.safe_capture,
                strict_capture: args.strict_capture,
                safe_capture: args.safe_capture,
                resource_budget: args.resource_budget.as_str().to_string(),
                block_third_party: args.block_third_party,
                block_analytics: args.block_analytics,
                block_media: args.block_media,
                block_fonts: args.block_fonts,
                block_images: args.block_images,
                allow_active: args.allow_active,
                click_selector: args.click_selector,
                auth_state: args.auth_state,
                screenshot_dir: args.screenshot_dir,
                artifact_dir: Some(args.artifact_dir),
                keep_artifacts: args.keep_artifacts,
                redact_text: args.redact_text,
                hash_text: !args.redact_text && !args.include_text || args.hash_text,
                include_text: args.include_text,
                redact_attributes: args.redact_attributes,
                redact_images: args.redact_images,
                safe_interactions: true,
                no_click: true,
                no_form_submit: true,
                auto_install_deps: !args.no_install_deps,
                deps_dir: Some(args.deps_dir),
                playwright_browser: args.playwright_browser,
            };
            let raw = capture_with_optional_viewports(&config)?;
            let analysis = AnalysisConfig {
                scope: args.scope.as_str().to_string(),
                detail: args.detail.as_str().to_string(),
                format: args.format.as_str().to_string(),
            };
            let output_model =
                analyze_raw_facts(raw, analysis).map_err(CliError::analysis_failure)?;
            match args.format {
                args::OutputFormat::Json | args::OutputFormat::ReportJson => {
                    output::write_json(&output_model, &args.output, args.pretty)
                }
                args::OutputFormat::ReconstructionJson => {
                    output::write_json(&output_model.reconstruction, &args.output, args.pretty)
                }
                args::OutputFormat::W3cTokens => {
                    output::write_json(&output_model.tokens, &args.output, args.pretty)
                }
                args::OutputFormat::CssVars => {
                    let css = css_vars::emit_css_vars(&output_model.tokens);
                    output::write_string(&css, &args.output)
                }
                args::OutputFormat::Tailwind => {
                    let config = tailwind::emit_tailwind_config(&output_model.tokens);
                    output::write_string(&config, &args.output)
                }
                args::OutputFormat::Raw => Err(anyhow::anyhow!(
                    "--format raw is supported by the capture command"
                )),
            }
            .map_err(CliError::output_failure)
        }
        Commands::Capture(args) => {
            let config = CaptureConfig {
                url: args.url,
                viewport: args.viewport,
                viewports: args.viewports,
                color_scheme: args.color_scheme.as_str().to_string(),
                states: args.states,
                include_screenshots: args.include_screenshots,
                screenshot: args.screenshot.as_str().to_string(),
                wait: if args.safe_capture {
                    "auto".to_string()
                } else {
                    args.wait.as_str().to_string()
                },
                wait_for_selector: args.wait_for_selector,
                timeout_ms: args.timeout_ms,
                navigation_timeout_ms: args.navigation_timeout_ms,
                capture_timeout_ms: args.capture_timeout_ms,
                stability_window_ms: args.stability_window_ms,
                max_stability_wait_ms: args.max_stability_wait_ms,
                ignore_networkidle_timeout: args.ignore_networkidle_timeout,
                capture_on_timeout: args.capture_on_timeout || args.safe_capture,
                strict_capture: args.strict_capture,
                safe_capture: args.safe_capture,
                resource_budget: args.resource_budget.as_str().to_string(),
                block_third_party: args.block_third_party,
                block_analytics: args.block_analytics,
                block_media: args.block_media,
                block_fonts: args.block_fonts,
                block_images: args.block_images,
                allow_active: args.allow_active,
                click_selector: args.click_selector,
                auth_state: args.auth_state,
                screenshot_dir: args.screenshot_dir,
                artifact_dir: Some(args.artifact_dir),
                keep_artifacts: args.keep_artifacts,
                redact_text: args.redact_text,
                hash_text: !args.redact_text && !args.include_text || args.hash_text,
                include_text: args.include_text,
                redact_attributes: args.redact_attributes,
                redact_images: args.redact_images,
                safe_interactions: true,
                no_click: true,
                no_form_submit: true,
                auto_install_deps: !args.no_install_deps,
                deps_dir: Some(args.deps_dir),
                playwright_browser: args.playwright_browser,
            };
            let raw = capture_with_optional_viewports(&config)?;
            let output_path = args.raw_output.as_deref().unwrap_or("-");
            output::write_json(&raw, output_path, args.pretty).map_err(CliError::output_failure)
        }
        Commands::Analyze(args) => {
            let raw = read_raw_facts(&args.input)?;
            let analysis = AnalysisConfig {
                scope: "page".to_string(),
                detail: "all".to_string(),
                format: args.format.as_str().to_string(),
            };
            let output_model =
                analyze_raw_facts(raw, analysis).map_err(CliError::analysis_failure)?;
            match args.format {
                args::OutputFormat::Json | args::OutputFormat::ReportJson => {
                    output::write_json(&output_model, &args.output, args.pretty)
                }
                args::OutputFormat::ReconstructionJson => {
                    output::write_json(&output_model.reconstruction, &args.output, args.pretty)
                }
                args::OutputFormat::W3cTokens => {
                    output::write_json(&output_model.tokens, &args.output, args.pretty)
                }
                args::OutputFormat::CssVars => output::write_string(
                    &css_vars::emit_css_vars(&output_model.tokens),
                    &args.output,
                ),
                args::OutputFormat::Tailwind => output::write_string(
                    &tailwind::emit_tailwind_config(&output_model.tokens),
                    &args.output,
                ),
                args::OutputFormat::Raw => {
                    output::write_json(&output_model.evidence, &args.output, args.pretty)
                }
            }
            .map_err(CliError::output_failure)
        }
        Commands::Tokens(args) => {
            let raw = read_raw_facts(&args.input)?;
            let tokens = tokens_from_raw_facts(raw).map_err(CliError::analysis_failure)?;
            match args.format {
                args::OutputFormat::W3cTokens
                | args::OutputFormat::Json
                | args::OutputFormat::ReportJson
                | args::OutputFormat::ReconstructionJson => {
                    output::write_json(&tokens, &args.output, args.pretty)
                }
                args::OutputFormat::CssVars => {
                    output::write_string(&css_vars::emit_css_vars(&tokens), &args.output)
                }
                args::OutputFormat::Tailwind => {
                    output::write_string(&tailwind::emit_tailwind_config(&tokens), &args.output)
                }
                args::OutputFormat::Raw => {
                    Err(anyhow::anyhow!("--format raw is not a token output format"))
                }
            }
            .map_err(CliError::output_failure)
        }
        Commands::Diff(args) => {
            let before = fs::read(&args.before).map_err(CliError::generic)?;
            let after = fs::read(&args.after).map_err(CliError::generic)?;
            let before_hash = hex_sha256(&before);
            let after_hash = hex_sha256(&after);
            let payload = json!({
                "schema_version": "style-scraper-diff.v1",
                "changed": before_hash != after_hash,
                "severity": if before_hash == after_hash { "none" } else { "moderate" },
                "before_hash": format!("sha256:{before_hash}"),
                "after_hash": format!("sha256:{after_hash}"),
                "diagnostics": {
                    "warnings": [{
                        "code": "DIFF_MVP_HASH_ONLY",
                        "message": "Initial diff implementation compares canonical file bytes only; token-aware drift detection is planned.",
                        "phase": "diff",
                        "severity": "warning"
                    }]
                }
            });
            output::write_json(&payload, &args.output, args.pretty)
                .map_err(CliError::output_failure)
        }
        Commands::DiffVisual(args) => {
            let payload = diff_visual_payload(&args.before, &args.after, args.tolerance)?;
            output::write_json(&payload, &args.output, args.pretty)
                .map_err(CliError::output_failure)
        }
        Commands::Validate(args) => {
            let bytes = fs::read(&args.input).map_err(CliError::generic)?;
            let raw_result: Result<RawFacts, _> = serde_json::from_slice(&bytes);
            let payload = match raw_result {
                Ok(raw) => match style_scraper_core::validate::schema::validate_raw_facts(&raw) {
                    Ok(()) => json!({
                        "schema_version": "style-scraper-validation.v1",
                        "valid": true,
                        "document_type": "raw-facts",
                        "diagnostics": []
                    }),
                    Err(error) => {
                        return Err(CliError::schema_failure(error));
                    }
                },
                Err(_) => {
                    let value: serde_json::Value =
                        serde_json::from_slice(&bytes).map_err(CliError::schema_failure)?;
                    let valid = value
                        .get("schema_version")
                        .and_then(|value| value.as_str())
                        .is_some_and(|version| version == "style-scraper-output.v1");
                    json!({
                        "schema_version": "style-scraper-validation.v1",
                        "valid": valid,
                        "document_type": if valid { "style-scraper-output" } else { "unknown" },
                        "diagnostics": if valid { json!([]) } else { json!([{
                            "code": "UNKNOWN_SCHEMA",
                            "message": "Document is neither RawFacts nor style-scraper output.",
                            "phase": "validate",
                            "severity": "error"
                        }]) }
                    })
                }
            };
            output::write_json(&payload, &args.output, args.pretty)
                .map_err(CliError::output_failure)
        }
        Commands::Crawl(args) => {
            let payload = json!({
                "schema_version": "style-scraper-crawl.v1",
                "url": args.url,
                "max_pages": args.max_pages,
                "max_depth": args.max_depth,
                "same_origin": args.same_origin,
                "respect_robots": args.respect_robots,
                "delay_ms": args.delay_ms,
                "parallel_pages": args.parallel_pages.min(3),
                "include_path": args.include_path,
                "exclude_path": args.exclude_path,
                "dangerous_route_blocklist": ["logout", "checkout", "cart", "delete", "admin"],
                "rate_limit_policy": {
                    "stop_on": [429, 403, 503],
                    "respect_retry_after": true,
                    "stealth_evasion": false,
                    "proxy_rotation": false,
                    "captcha_bypass": false
                },
                "status": "not_implemented",
                "diagnostics": [{
                    "code": "CRAWL_NOT_IMPLEMENTED",
                    "message": "The safe crawl contract is defined, but route discovery and aggregation are not enabled yet. Use page scope or capture/analyze until crawl implementation lands.",
                    "phase": "crawl",
                    "severity": "warning"
                }]
            });
            output::write_json(&payload, &args.output, args.pretty)
                .map_err(CliError::output_failure)
        }
    }
}

fn read_raw_facts(path: &str) -> Result<RawFacts, CliError> {
    let bytes = fs::read(path).map_err(CliError::generic)?;
    serde_json::from_slice(&bytes).map_err(CliError::schema_failure)
}

fn capture_with_optional_viewports(config: &CaptureConfig) -> Result<RawFacts, CliError> {
    let runner = BunProbeRunner::development_default();
    if config.viewports.is_empty() {
        return runner.capture(config).map_err(CliError::from_probe);
    }

    let mut aggregate: Option<RawFacts> = None;
    for viewport in &config.viewports {
        let mut per_viewport = config.clone();
        per_viewport.viewport = viewport.clone();
        per_viewport.viewports = Vec::new();
        let mut raw = runner
            .capture(&per_viewport)
            .map_err(CliError::from_probe)?;
        if let Some(base) = &mut aggregate {
            base.pages.append(&mut raw.pages);
            base.diagnostics.append(&mut raw.diagnostics);
        } else {
            aggregate = Some(raw);
        }
    }

    aggregate.ok_or_else(|| CliError::generic("--viewports did not contain any viewport values"))
}

fn diff_visual_payload(
    before_path: &str,
    after_path: &str,
    tolerance: f64,
) -> Result<serde_json::Value, CliError> {
    let before = image::open(before_path)
        .map_err(CliError::generic)?
        .to_rgba8();
    let after = image::open(after_path)
        .map_err(CliError::generic)?
        .to_rgba8();
    let (before_width, before_height) = before.dimensions();
    let (after_width, after_height) = after.dimensions();
    if (before_width, before_height) != (after_width, after_height) {
        let payload = json!({
            "schema_version": "style-scraper-visual-diff.v1",
            "visual_fidelity": {
                "pixel_diff_ratio": 1.0,
                "ssim": null,
                "score": 0.0,
                "verdict": "dimension-mismatch"
            },
            "dimensions": {
                "before": { "width": before_width, "height": before_height },
                "after": { "width": after_width, "height": after_height }
            },
            "diagnostics": [{
                "code": "VISUAL_DIFF_DIMENSION_MISMATCH",
                "message": "Images must have identical dimensions for deterministic pixel diff.",
                "phase": "diff-visual",
                "severity": "error"
            }]
        });
        return Ok(payload);
    }

    let mut differing_pixels = 0_u64;
    let mut min_x = before_width;
    let mut min_y = before_height;
    let mut max_x = 0_u32;
    let mut max_y = 0_u32;
    let channel_tolerance = (tolerance.clamp(0.0, 1.0) * 255.0).round() as i16;
    for y in 0..before_height {
        for x in 0..before_width {
            let left = before.get_pixel(x, y).0;
            let right = after.get_pixel(x, y).0;
            let different = left
                .iter()
                .zip(right.iter())
                .any(|(l, r)| (*l as i16 - *r as i16).abs() > channel_tolerance);
            if different {
                differing_pixels += 1;
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
            }
        }
    }

    let total_pixels = u64::from(before_width) * u64::from(before_height);
    let pixel_diff_ratio = if total_pixels == 0 {
        0.0
    } else {
        differing_pixels as f64 / total_pixels as f64
    };
    let ssim = compute_global_ssim(&before, &after);
    let score = ((1.0 - pixel_diff_ratio) * 0.45 + ssim * 0.55).clamp(0.0, 1.0);
    let verdict = if pixel_diff_ratio <= 0.01 && ssim >= 0.985 {
        "near-identical"
    } else if pixel_diff_ratio <= 0.05 && ssim >= 0.95 {
        "high"
    } else if pixel_diff_ratio <= 0.15 && ssim >= 0.85 {
        "medium"
    } else {
        "low"
    };
    let changed_bounds = if differing_pixels == 0 {
        serde_json::Value::Null
    } else {
        json!({
            "x": min_x,
            "y": min_y,
            "width": max_x.saturating_sub(min_x) + 1,
            "height": max_y.saturating_sub(min_y) + 1
        })
    };

    Ok(json!({
        "schema_version": "style-scraper-visual-diff.v1",
        "visual_fidelity": {
            "pixel_diff_ratio": pixel_diff_ratio,
            "ssim": ssim,
            "score": score,
            "verdict": verdict,
            "thresholds": {
                "pixel_diff_ratio_high_max": 0.05,
                "ssim_high_min": 0.95
            }
        },
        "dimensions": {
            "width": before_width,
            "height": before_height
        },
        "largest_changed_region": changed_bounds,
        "anti_aliasing_tolerance": tolerance,
        "diagnostics": [{
            "code": "GLOBAL_SSIM_IMPLEMENTED",
            "message": "Global luminance SSIM is implemented as a deterministic fidelity metric; local windowed SSIM can be added later for regional scoring.",
            "phase": "diff-visual",
            "severity": "info"
        }]
    }))
}

fn compute_global_ssim(
    before: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    after: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
) -> f64 {
    let mut left_values = Vec::with_capacity(before.len() / 4);
    let mut right_values = Vec::with_capacity(after.len() / 4);
    for (left, right) in before.pixels().zip(after.pixels()) {
        left_values.push(luminance(left.0));
        right_values.push(luminance(right.0));
    }
    if left_values.is_empty() {
        return 1.0;
    }
    let n = left_values.len() as f64;
    let left_mean = left_values.iter().sum::<f64>() / n;
    let right_mean = right_values.iter().sum::<f64>() / n;
    let mut left_var = 0.0;
    let mut right_var = 0.0;
    let mut covariance = 0.0;
    for (left, right) in left_values.iter().zip(right_values.iter()) {
        left_var += (left - left_mean).powi(2);
        right_var += (right - right_mean).powi(2);
        covariance += (left - left_mean) * (right - right_mean);
    }
    let denominator_n = (n - 1.0).max(1.0);
    left_var /= denominator_n;
    right_var /= denominator_n;
    covariance /= denominator_n;

    let c1 = (0.01_f64).powi(2);
    let c2 = (0.03_f64).powi(2);
    let numerator = (2.0 * left_mean * right_mean + c1) * (2.0 * covariance + c2);
    let denominator = (left_mean.powi(2) + right_mean.powi(2) + c1) * (left_var + right_var + c2);
    if denominator == 0.0 {
        1.0
    } else {
        (numerator / denominator).clamp(0.0, 1.0)
    }
}

fn luminance(pixel: [u8; 4]) -> f64 {
    let r = f64::from(pixel[0]) / 255.0;
    let g = f64::from(pixel[1]) / 255.0;
    let b = f64::from(pixel[2]) / 255.0;
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

#[derive(Debug)]
struct CliError {
    exit_code: i32,
    code: &'static str,
    message: String,
    phase: &'static str,
    recoverable: bool,
}

impl CliError {
    fn generic(error: impl std::fmt::Display) -> Self {
        Self {
            exit_code: exit_codes::GENERIC_FAILURE,
            code: "GENERIC_FAILURE",
            message: error.to_string(),
            phase: "unknown",
            recoverable: false,
        }
    }

    fn analysis_failure(error: impl std::fmt::Display) -> Self {
        Self {
            exit_code: exit_codes::ANALYSIS_FAILURE,
            code: "ANALYSIS_FAILURE",
            message: error.to_string(),
            phase: "analysis",
            recoverable: false,
        }
    }

    fn schema_failure(error: impl std::fmt::Display) -> Self {
        Self {
            exit_code: exit_codes::SCHEMA_VALIDATION_FAILURE,
            code: "SCHEMA_VALIDATION_FAILURE",
            message: error.to_string(),
            phase: "validate",
            recoverable: true,
        }
    }

    fn output_failure(error: impl std::fmt::Display) -> Self {
        Self {
            exit_code: exit_codes::OUTPUT_WRITE_FAILURE,
            code: "OUTPUT_WRITE_FAILURE",
            message: error.to_string(),
            phase: "emit",
            recoverable: false,
        }
    }

    fn from_probe(error: ProbeRunnerError) -> Self {
        let code = error.code();
        let exit_code = error.exit_code();
        Self {
            exit_code,
            code,
            message: error.to_string(),
            phase: "capture",
            recoverable: true,
        }
    }

    fn to_stderr_payload(&self) -> ErrorPayload<'_> {
        ErrorPayload {
            error: ErrorBody {
                code: self.code,
                message: &self.message,
                phase: self.phase,
                recoverable: self.recoverable,
            },
        }
    }
}

#[derive(Debug, Serialize)]
struct ErrorPayload<'a> {
    error: ErrorBody<'a>,
}

#[derive(Debug, Serialize)]
struct ErrorBody<'a> {
    code: &'a str,
    message: &'a str,
    phase: &'a str,
    recoverable: bool,
}
