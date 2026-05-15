use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "style-scraper")]
#[command(about = "AI-agent-facing frontend design tokenization CLI")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Extract(ExtractArgs),
    Capture(CaptureArgs),
    Analyze(AnalyzeArgs),
    Tokens(TokensArgs),
    Diff(DiffArgs),
    DiffVisual(DiffVisualArgs),
    Validate(ValidateArgs),
    Crawl(CrawlArgs),
}

#[derive(Debug, Clone, Parser)]
pub struct ExtractArgs {
    #[arg(long)]
    pub url: String,
    #[arg(long, value_enum, default_value_t = Scope::Page)]
    pub scope: Scope,
    #[arg(long, value_enum, default_value_t = Detail::All)]
    pub detail: Detail,
    #[arg(long, value_enum, default_value_t = OutputFormat::Json)]
    pub format: OutputFormat,
    #[arg(long)]
    pub safe_capture: bool,
    #[arg(long, default_value = "1440x900")]
    pub viewport: String,
    #[arg(long, value_delimiter = ',')]
    pub viewports: Vec<String>,
    #[arg(long, value_enum, default_value_t = ColorScheme::Light)]
    pub color_scheme: ColorScheme,
    #[arg(long, value_delimiter = ',', default_value = "hover,focus-visible")]
    pub states: Vec<String>,
    #[arg(long)]
    pub include_screenshots: bool,
    #[arg(long, value_enum, default_value_t = ScreenshotMode::Viewport)]
    pub screenshot: ScreenshotMode,
    #[arg(long, value_enum, default_value_t = WaitMode::Auto)]
    pub wait: WaitMode,
    #[arg(long)]
    pub wait_for_selector: Option<String>,
    #[arg(long, default_value_t = 30_000)]
    pub timeout_ms: u64,
    #[arg(long, default_value_t = 15_000)]
    pub navigation_timeout_ms: u64,
    #[arg(long, default_value_t = 30_000)]
    pub capture_timeout_ms: u64,
    #[arg(long, default_value_t = 500)]
    pub stability_window_ms: u64,
    #[arg(long, default_value_t = 5_000)]
    pub max_stability_wait_ms: u64,
    #[arg(long, default_value_t = true)]
    pub ignore_networkidle_timeout: bool,
    #[arg(long, default_value_t = true)]
    pub capture_on_timeout: bool,
    #[arg(long)]
    pub strict_capture: bool,
    #[arg(long, value_enum, default_value_t = ResourceBudget::Balanced)]
    pub resource_budget: ResourceBudget,
    #[arg(long)]
    pub block_third_party: bool,
    #[arg(long)]
    pub block_analytics: bool,
    #[arg(long)]
    pub block_media: bool,
    #[arg(long)]
    pub block_fonts: bool,
    #[arg(long)]
    pub block_images: bool,
    #[arg(long)]
    pub allow_active: bool,
    #[arg(long)]
    pub click_selector: Option<String>,
    #[arg(long)]
    pub auth_state: Option<String>,
    #[arg(long)]
    pub screenshot_dir: Option<String>,
    #[arg(long, default_value = ".style-scraper/artifacts")]
    pub artifact_dir: String,
    #[arg(long)]
    pub keep_artifacts: bool,
    #[arg(long)]
    pub redact_text: bool,
    #[arg(long)]
    pub hash_text: bool,
    #[arg(long)]
    pub include_text: bool,
    #[arg(long)]
    pub redact_attributes: bool,
    #[arg(long)]
    pub redact_images: bool,
    #[arg(long)]
    pub safe_interactions: bool,
    #[arg(long)]
    pub no_click: bool,
    #[arg(long)]
    pub no_form_submit: bool,
    #[arg(long)]
    pub no_install_deps: bool,
    #[arg(long, default_value = ".style-scraper/deps")]
    pub deps_dir: String,
    #[arg(long, default_value = "chromium")]
    pub playwright_browser: String,
    #[arg(long, default_value = "-")]
    pub output: String,
    #[arg(long)]
    pub pretty: bool,
}

#[derive(Debug, Clone, Parser)]
pub struct CaptureArgs {
    #[arg(long)]
    pub url: String,
    #[arg(long)]
    pub raw_output: Option<String>,
    #[arg(long)]
    pub safe_capture: bool,
    #[arg(long, default_value = "1440x900")]
    pub viewport: String,
    #[arg(long, value_delimiter = ',')]
    pub viewports: Vec<String>,
    #[arg(long, value_enum, default_value_t = ColorScheme::Light)]
    pub color_scheme: ColorScheme,
    #[arg(long, value_delimiter = ',', default_value = "hover,focus-visible")]
    pub states: Vec<String>,
    #[arg(long)]
    pub include_screenshots: bool,
    #[arg(long, value_enum, default_value_t = ScreenshotMode::Viewport)]
    pub screenshot: ScreenshotMode,
    #[arg(long, value_enum, default_value_t = WaitMode::Auto)]
    pub wait: WaitMode,
    #[arg(long)]
    pub wait_for_selector: Option<String>,
    #[arg(long, default_value_t = 30_000)]
    pub timeout_ms: u64,
    #[arg(long, default_value_t = 15_000)]
    pub navigation_timeout_ms: u64,
    #[arg(long, default_value_t = 30_000)]
    pub capture_timeout_ms: u64,
    #[arg(long, default_value_t = 500)]
    pub stability_window_ms: u64,
    #[arg(long, default_value_t = 5_000)]
    pub max_stability_wait_ms: u64,
    #[arg(long, default_value_t = true)]
    pub ignore_networkidle_timeout: bool,
    #[arg(long, default_value_t = true)]
    pub capture_on_timeout: bool,
    #[arg(long)]
    pub strict_capture: bool,
    #[arg(long, value_enum, default_value_t = ResourceBudget::Balanced)]
    pub resource_budget: ResourceBudget,
    #[arg(long)]
    pub block_third_party: bool,
    #[arg(long)]
    pub block_analytics: bool,
    #[arg(long)]
    pub block_media: bool,
    #[arg(long)]
    pub block_fonts: bool,
    #[arg(long)]
    pub block_images: bool,
    #[arg(long)]
    pub allow_active: bool,
    #[arg(long)]
    pub click_selector: Option<String>,
    #[arg(long)]
    pub auth_state: Option<String>,
    #[arg(long)]
    pub screenshot_dir: Option<String>,
    #[arg(long, default_value = ".style-scraper/artifacts")]
    pub artifact_dir: String,
    #[arg(long)]
    pub keep_artifacts: bool,
    #[arg(long)]
    pub redact_text: bool,
    #[arg(long)]
    pub hash_text: bool,
    #[arg(long)]
    pub include_text: bool,
    #[arg(long)]
    pub redact_attributes: bool,
    #[arg(long)]
    pub redact_images: bool,
    #[arg(long)]
    pub safe_interactions: bool,
    #[arg(long)]
    pub no_click: bool,
    #[arg(long)]
    pub no_form_submit: bool,
    #[arg(long)]
    pub no_install_deps: bool,
    #[arg(long, default_value = ".style-scraper/deps")]
    pub deps_dir: String,
    #[arg(long, default_value = "chromium")]
    pub playwright_browser: String,
    #[arg(long)]
    pub pretty: bool,
}

#[derive(Debug, Clone, Parser)]
pub struct AnalyzeArgs {
    #[arg(long)]
    pub input: String,
    #[arg(long, value_enum, default_value_t = OutputFormat::Json)]
    pub format: OutputFormat,
    #[arg(long, default_value = "-")]
    pub output: String,
    #[arg(long)]
    pub pretty: bool,
}

#[derive(Debug, Clone, Parser)]
pub struct TokensArgs {
    #[arg(long)]
    pub input: String,
    #[arg(long, value_enum, default_value_t = OutputFormat::W3cTokens)]
    pub format: OutputFormat,
    #[arg(long, default_value = "-")]
    pub output: String,
    #[arg(long)]
    pub pretty: bool,
}

#[derive(Debug, Clone, Parser)]
pub struct DiffArgs {
    #[arg(long)]
    pub before: String,
    #[arg(long)]
    pub after: String,
    #[arg(long, default_value = "-")]
    pub output: String,
    #[arg(long)]
    pub pretty: bool,
}

#[derive(Debug, Clone, Parser)]
pub struct DiffVisualArgs {
    #[arg(long)]
    pub before: String,
    #[arg(long)]
    pub after: String,
    #[arg(long, value_enum, default_value_t = OutputFormat::Json)]
    pub format: OutputFormat,
    #[arg(long, default_value_t = 0.02)]
    pub tolerance: f64,
    #[arg(long, default_value = "-")]
    pub output: String,
    #[arg(long)]
    pub pretty: bool,
}

#[derive(Debug, Clone, Parser)]
pub struct ValidateArgs {
    #[arg(long)]
    pub input: String,
    #[arg(long, default_value = "-")]
    pub output: String,
    #[arg(long)]
    pub pretty: bool,
}

#[derive(Debug, Clone, Parser)]
pub struct CrawlArgs {
    #[arg(long)]
    pub url: String,
    #[arg(long, default_value_t = 25)]
    pub max_pages: usize,
    #[arg(long, default_value_t = 2)]
    pub max_depth: usize,
    #[arg(long, default_value_t = true)]
    pub same_origin: bool,
    #[arg(long, default_value_t = true)]
    pub respect_robots: bool,
    #[arg(long, default_value_t = 1000)]
    pub delay_ms: u64,
    #[arg(long, default_value_t = 1)]
    pub parallel_pages: usize,
    #[arg(long)]
    pub include_path: Vec<String>,
    #[arg(long)]
    pub exclude_path: Vec<String>,
    #[arg(long, default_value = "-")]
    pub output: String,
    #[arg(long)]
    pub pretty: bool,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, ValueEnum)]
pub enum Scope {
    Page,
    Crawl,
    Sitemap,
    RoutesFile,
    Storybook,
    ComponentLibrary,
}

impl Scope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Page => "page",
            Self::Crawl => "crawl",
            Self::Sitemap => "sitemap",
            Self::RoutesFile => "routes-file",
            Self::Storybook => "storybook",
            Self::ComponentLibrary => "component-library",
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, ValueEnum)]
pub enum Detail {
    Atomic,
    Molecule,
    Organism,
    Template,
    Page,
    All,
}

impl Detail {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Atomic => "atomic",
            Self::Molecule => "molecule",
            Self::Organism => "organism",
            Self::Template => "template",
            Self::Page => "page",
            Self::All => "all",
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, ValueEnum)]
pub enum OutputFormat {
    Json,
    W3cTokens,
    Raw,
    CssVars,
    Tailwind,
    ReportJson,
    ReconstructionJson,
}

impl OutputFormat {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::W3cTokens => "w3c-tokens",
            Self::Raw => "raw",
            Self::CssVars => "css-vars",
            Self::Tailwind => "tailwind",
            Self::ReportJson => "report-json",
            Self::ReconstructionJson => "reconstruction-json",
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, ValueEnum)]
pub enum ScreenshotMode {
    Viewport,
    FullPage,
    Elements,
}

impl ScreenshotMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Viewport => "viewport",
            Self::FullPage => "full-page",
            Self::Elements => "elements",
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, ValueEnum)]
pub enum ColorScheme {
    Light,
    Dark,
}

impl ColorScheme {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, ValueEnum)]
pub enum WaitMode {
    Domcontentloaded,
    Load,
    Networkidle,
    Stable,
    Selector,
    Auto,
}

impl WaitMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Domcontentloaded => "domcontentloaded",
            Self::Load => "load",
            Self::Networkidle => "networkidle",
            Self::Stable => "stable",
            Self::Selector => "selector",
            Self::Auto => "auto",
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, ValueEnum)]
pub enum ResourceBudget {
    Safe,
    Balanced,
    Full,
}

impl ResourceBudget {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Safe => "safe",
            Self::Balanced => "balanced",
            Self::Full => "full",
        }
    }
}
