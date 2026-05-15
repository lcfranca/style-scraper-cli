use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProbeRunnerError {
    #[error("Bun runtime was not found. Install Bun or use the Docker image.")]
    BunNotFound,
    #[error("Playwright dependency was not found in the probe package. Run `cd probe && bun install` or use the Docker image.")]
    PlaywrightNotFound,
    #[error("Playwright browser executable was not found. Run `cd probe && bunx playwright install chromium` or use the Docker image.")]
    BrowserNotInstalled,
    #[error("automatic installation is not supported for {dependency}: {message}")]
    UnsupportedAutoInstall {
        dependency: &'static str,
        message: String,
    },
    #[error("failed to install {dependency}: {message}")]
    DependencyInstallFailed {
        dependency: &'static str,
        message: String,
    },
    #[error("Bun probe timed out after {timeout_ms}ms: {stderr}")]
    Timeout { timeout_ms: u64, stderr: String },
    #[error("failed to spawn Bun probe: {0}")]
    SpawnFailed(String),
    #[error("Bun probe failed with exit code {exit_code}: {stderr}")]
    ProbeFailed { exit_code: i32, stderr: String },
    #[error("probe stdout was not valid RawFacts JSON: {0}")]
    InvalidJson(String),
    #[error("probe returned invalid RawFacts: {0}")]
    InvalidRawFacts(String),
}

impl ProbeRunnerError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::BunNotFound => "BUN_NOT_FOUND",
            Self::PlaywrightNotFound => "PLAYWRIGHT_NOT_FOUND",
            Self::BrowserNotInstalled => "BROWSER_NOT_INSTALLED",
            Self::UnsupportedAutoInstall { .. } => "UNSUPPORTED_AUTO_INSTALL",
            Self::DependencyInstallFailed { .. } => "DEPENDENCY_INSTALL_FAILED",
            Self::Timeout { .. } => "NAVIGATION_TIMEOUT",
            Self::SpawnFailed(_) => "PROBE_SPAWN_FAILED",
            Self::ProbeFailed { .. } => "PROBE_FAILURE",
            Self::InvalidJson(_) => "PROBE_INVALID_JSON",
            Self::InvalidRawFacts(_) => "SCHEMA_VALIDATION_FAILURE",
        }
    }

    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Timeout { .. } => 4,
            Self::InvalidJson(_) | Self::InvalidRawFacts(_) => 5,
            _ => 3,
        }
    }
}
