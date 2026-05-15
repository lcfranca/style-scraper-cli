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
                color_scheme: args.color_scheme.as_str().to_string(),
                states: args.states,
                include_screenshots: args.include_screenshots,
                wait: args.wait.as_str().to_string(),
                timeout_ms: args.timeout_ms,
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
            let raw = BunProbeRunner::development_default()
                .capture(&config)
                .map_err(CliError::from_probe)?;
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
                color_scheme: args.color_scheme.as_str().to_string(),
                states: args.states,
                include_screenshots: args.include_screenshots,
                wait: args.wait.as_str().to_string(),
                timeout_ms: args.timeout_ms,
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
            let raw = BunProbeRunner::development_default()
                .capture(&config)
                .map_err(CliError::from_probe)?;
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
                | args::OutputFormat::ReportJson => {
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
