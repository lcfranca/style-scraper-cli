use crate::errors::ProbeRunnerError;
use style_scraper_core::model::raw::RawFacts;

#[derive(Debug, Clone)]
pub struct CaptureConfig {
    pub url: String,
    pub viewport: String,
    pub color_scheme: String,
    pub states: Vec<String>,
    pub include_screenshots: bool,
    pub wait: String,
    pub timeout_ms: u64,
    pub auth_state: Option<String>,
    pub screenshot_dir: Option<String>,
    pub artifact_dir: Option<String>,
    pub keep_artifacts: bool,
    pub redact_text: bool,
    pub hash_text: bool,
    pub include_text: bool,
    pub redact_attributes: bool,
    pub redact_images: bool,
    pub safe_interactions: bool,
    pub no_click: bool,
    pub no_form_submit: bool,
    pub auto_install_deps: bool,
    pub deps_dir: Option<String>,
    pub playwright_browser: String,
}

impl Default for CaptureConfig {
    fn default() -> Self {
        Self {
            url: String::new(),
            viewport: "1440x900".to_string(),
            color_scheme: "light".to_string(),
            states: Vec::new(),
            include_screenshots: false,
            wait: "stable".to_string(),
            timeout_ms: 30_000,
            auth_state: None,
            screenshot_dir: None,
            artifact_dir: Some(".style-scraper/artifacts".to_string()),
            keep_artifacts: false,
            redact_text: false,
            hash_text: true,
            include_text: false,
            redact_attributes: false,
            redact_images: false,
            safe_interactions: true,
            no_click: true,
            no_form_submit: true,
            auto_install_deps: true,
            deps_dir: Some(".style-scraper/deps".to_string()),
            playwright_browser: "chromium".to_string(),
        }
    }
}

pub trait ProbeRunner {
    fn capture(&self, config: &CaptureConfig) -> Result<RawFacts, ProbeRunnerError>;
}
