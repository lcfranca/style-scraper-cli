use crate::command::{CaptureConfig, ProbeRunner};
use crate::errors::ProbeRunnerError;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use style_scraper_core::model::raw::RawFacts;
use style_scraper_core::validate::schema::validate_raw_facts;

const BUN_VERSION: &str = "1.1.6";
const INSTALL_TIMEOUT_MS: u64 = 300_000;

#[derive(Debug, Clone)]
pub struct BunProbeRunner {
    bun_binary: String,
    probe_dir: PathBuf,
    entrypoint: PathBuf,
}

impl BunProbeRunner {
    pub fn development_default() -> Self {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let workspace_root = manifest_dir
            .parent()
            .and_then(|path| path.parent())
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));
        let probe_dir = workspace_root.join("probe");
        Self {
            bun_binary: "bun".to_string(),
            entrypoint: probe_dir.join("src").join("capture.ts"),
            probe_dir,
        }
    }

    pub fn with_paths(
        bun_binary: impl Into<String>,
        probe_dir: impl Into<PathBuf>,
        entrypoint: impl Into<PathBuf>,
    ) -> Self {
        Self {
            bun_binary: bun_binary.into(),
            probe_dir: probe_dir.into(),
            entrypoint: entrypoint.into(),
        }
    }
}

impl ProbeRunner for BunProbeRunner {
    fn capture(&self, config: &CaptureConfig) -> Result<RawFacts, ProbeRunnerError> {
        let bun_binary = self.resolve_bun_binary(config)?;
        self.ensure_probe_dependencies_available(&bun_binary, config)?;
        self.ensure_playwright_browser_available(&bun_binary, config)?;

        let temp = ProbeTempFiles::new(config)?;
        let mut command = Command::new(&bun_binary);
        command
            .current_dir(&self.probe_dir)
            .arg("run")
            .arg(&self.entrypoint)
            .arg("--url")
            .arg(&config.url)
            .arg("--viewport")
            .arg(&config.viewport)
            .arg("--color-scheme")
            .arg(&config.color_scheme)
            .arg("--wait")
            .arg(&config.wait)
            .arg("--timeout-ms")
            .arg(config.timeout_ms.to_string())
            .arg("--output-file")
            .arg(&temp.raw_path)
            .env("PLAYWRIGHT_BROWSERS_PATH", playwright_browsers_dir(config));

        if !config.states.is_empty() {
            command.arg("--states").arg(config.states.join(","));
        }
        if config.include_screenshots {
            command.arg("--include-screenshots");
        }
        if let Some(auth_state) = &config.auth_state {
            command.arg("--auth-state").arg(auth_state);
        }
        if let Some(screenshot_dir) = &config.screenshot_dir {
            command.arg("--screenshot-dir").arg(screenshot_dir);
        }
        if config.redact_text {
            command.arg("--redact-text");
        }
        if config.hash_text {
            command.arg("--hash-text");
        }
        if config.include_text {
            command.arg("--include-text");
        }
        if config.redact_attributes {
            command.arg("--redact-attributes");
        }
        if config.redact_images {
            command.arg("--redact-images");
        }
        if config.safe_interactions {
            command.arg("--safe-interactions");
        }
        if config.no_click {
            command.arg("--no-click");
        }
        if config.no_form_submit {
            command.arg("--no-form-submit");
        }

        let stdout_file = File::create(&temp.stdout_path)
            .map_err(|error| ProbeRunnerError::SpawnFailed(error.to_string()))?;
        let stderr_file = File::create(&temp.stderr_path)
            .map_err(|error| ProbeRunnerError::SpawnFailed(error.to_string()))?;

        let mut child = command
            .stdout(Stdio::from(stdout_file))
            .stderr(Stdio::from(stderr_file))
            .spawn()
            .map_err(|error| ProbeRunnerError::SpawnFailed(error.to_string()))?;

        let started = Instant::now();
        let status = loop {
            if let Some(status) = child
                .try_wait()
                .map_err(|error| ProbeRunnerError::SpawnFailed(error.to_string()))?
            {
                break status;
            }

            if started.elapsed() >= Duration::from_millis(config.timeout_ms) {
                let _ = child.kill();
                let _ = child.wait();
                let stderr = read_lossy(&temp.stderr_path);
                temp.cleanup(config.keep_artifacts);
                return Err(ProbeRunnerError::Timeout {
                    timeout_ms: config.timeout_ms,
                    stderr,
                });
            }

            std::thread::sleep(Duration::from_millis(25));
        };

        if !status.success() {
            let stderr = read_lossy(&temp.stderr_path);
            temp.cleanup(config.keep_artifacts);
            if is_browser_missing_error(&stderr) {
                return Err(ProbeRunnerError::BrowserNotInstalled);
            }
            if is_playwright_missing_error(&stderr) {
                return Err(ProbeRunnerError::PlaywrightNotFound);
            }
            return Err(ProbeRunnerError::ProbeFailed {
                exit_code: status.code().unwrap_or(3),
                stderr,
            });
        }

        let raw_bytes = fs::read(&temp.raw_path)
            .or_else(|_| fs::read(&temp.stdout_path))
            .map_err(|error| ProbeRunnerError::InvalidJson(error.to_string()))?;
        let raw: RawFacts = serde_json::from_slice(&raw_bytes)
            .map_err(|error| ProbeRunnerError::InvalidJson(error.to_string()))?;
        validate_raw_facts(&raw)
            .map_err(|error| ProbeRunnerError::InvalidRawFacts(error.to_string()))?;
        temp.cleanup(config.keep_artifacts);
        Ok(raw)
    }
}

impl BunProbeRunner {
    fn resolve_bun_binary(&self, config: &CaptureConfig) -> Result<PathBuf, ProbeRunnerError> {
        match Command::new(&self.bun_binary).arg("--version").output() {
            Ok(output) if output.status.success() => return Ok(PathBuf::from(&self.bun_binary)),
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(ProbeRunnerError::SpawnFailed(error.to_string())),
        }

        let local = local_bun_binary(config)?;
        if local.exists() {
            return Ok(local);
        }

        if !config.auto_install_deps {
            return Err(ProbeRunnerError::BunNotFound);
        }

        install_local_bun(config)?;
        let local = local_bun_binary(config)?;
        if local.exists() {
            Ok(local)
        } else {
            Err(ProbeRunnerError::DependencyInstallFailed {
                dependency: "bun",
                message: "installer completed but the Bun binary was not found".to_string(),
            })
        }
    }

    fn ensure_probe_dependencies_available(
        &self,
        bun_binary: &Path,
        config: &CaptureConfig,
    ) -> Result<(), ProbeRunnerError> {
        if self
            .probe_dir
            .join("node_modules")
            .join("playwright")
            .exists()
        {
            return Ok(());
        }

        if !config.auto_install_deps {
            return Err(ProbeRunnerError::PlaywrightNotFound);
        }

        let mut command = Command::new(bun_binary);
        command
            .current_dir(&self.probe_dir)
            .arg("install")
            .env("PLAYWRIGHT_BROWSERS_PATH", playwright_browsers_dir(config));
        run_install_command(command, INSTALL_TIMEOUT_MS, "probe npm dependencies")?;

        if self
            .probe_dir
            .join("node_modules")
            .join("playwright")
            .exists()
        {
            Ok(())
        } else {
            Err(ProbeRunnerError::DependencyInstallFailed {
                dependency: "playwright",
                message: "bun install completed but probe/node_modules/playwright is missing"
                    .to_string(),
            })
        }
    }

    fn ensure_playwright_browser_available(
        &self,
        bun_binary: &Path,
        config: &CaptureConfig,
    ) -> Result<(), ProbeRunnerError> {
        let browsers_dir = playwright_browsers_dir(config);
        if directory_has_entries(&browsers_dir) {
            return Ok(());
        }

        if !config.auto_install_deps {
            return Err(ProbeRunnerError::BrowserNotInstalled);
        }

        let mut command = Command::new(bun_binary);
        command
            .current_dir(&self.probe_dir)
            .arg("x")
            .arg("playwright")
            .arg("install")
            .arg(&config.playwright_browser)
            .env("PLAYWRIGHT_BROWSERS_PATH", &browsers_dir);
        run_install_command(command, INSTALL_TIMEOUT_MS, "playwright browser")?;

        if directory_has_entries(&browsers_dir) {
            Ok(())
        } else {
            Err(ProbeRunnerError::DependencyInstallFailed {
                dependency: "playwright-browser",
                message: "playwright install completed but no browser files were found".to_string(),
            })
        }
    }
}

fn install_local_bun(config: &CaptureConfig) -> Result<(), ProbeRunnerError> {
    let target = bun_release_target()?;
    let install_root = bun_install_root(config);
    fs::create_dir_all(&install_root).map_err(|error| {
        ProbeRunnerError::DependencyInstallFailed {
            dependency: "bun",
            message: error.to_string(),
        }
    })?;

    let archive = install_root.join(format!("bun-{target}.zip"));
    let url = format!(
        "https://github.com/oven-sh/bun/releases/download/bun-v{BUN_VERSION}/bun-{target}.zip"
    );

    let mut curl = Command::new("curl");
    curl.arg("-fsSL").arg(&url).arg("-o").arg(&archive);
    run_install_command(curl, INSTALL_TIMEOUT_MS, "bun runtime download")?;

    let mut unzip = Command::new("unzip");
    unzip
        .arg("-q")
        .arg("-o")
        .arg(&archive)
        .arg("-d")
        .arg(&install_root);
    run_install_command(unzip, INSTALL_TIMEOUT_MS, "bun runtime extraction")?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let binary = local_bun_binary(config)?;
        if binary.exists() {
            let mut permissions = fs::metadata(&binary)
                .map_err(|error| ProbeRunnerError::DependencyInstallFailed {
                    dependency: "bun",
                    message: error.to_string(),
                })?
                .permissions();
            permissions.set_mode(0o755);
            fs::set_permissions(&binary, permissions).map_err(|error| {
                ProbeRunnerError::DependencyInstallFailed {
                    dependency: "bun",
                    message: error.to_string(),
                }
            })?;
        }
    }

    Ok(())
}

fn run_install_command(
    mut command: Command,
    timeout_ms: u64,
    dependency: &'static str,
) -> Result<(), ProbeRunnerError> {
    let stderr_path = std::env::temp_dir().join(format!(
        "style-scraper-install-{}-{}.stderr",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0)
    ));
    let stderr_file =
        File::create(&stderr_path).map_err(|error| ProbeRunnerError::DependencyInstallFailed {
            dependency,
            message: error.to_string(),
        })?;

    let mut child = command
        .stdout(Stdio::null())
        .stderr(Stdio::from(stderr_file))
        .spawn()
        .map_err(|error| ProbeRunnerError::DependencyInstallFailed {
            dependency,
            message: error.to_string(),
        })?;

    let started = Instant::now();
    let status = loop {
        if let Some(status) =
            child
                .try_wait()
                .map_err(|error| ProbeRunnerError::DependencyInstallFailed {
                    dependency,
                    message: error.to_string(),
                })?
        {
            break status;
        }

        if started.elapsed() >= Duration::from_millis(timeout_ms) {
            let _ = child.kill();
            let _ = child.wait();
            let stderr = read_lossy(&stderr_path);
            let _ = fs::remove_file(&stderr_path);
            return Err(ProbeRunnerError::DependencyInstallFailed {
                dependency,
                message: format!("installation timed out after {timeout_ms}ms: {stderr}"),
            });
        }

        std::thread::sleep(Duration::from_millis(100));
    };

    let stderr = read_lossy(&stderr_path);
    let _ = fs::remove_file(&stderr_path);

    if status.success() {
        return Ok(());
    }

    Err(ProbeRunnerError::DependencyInstallFailed {
        dependency,
        message: format!(
            "command failed within {timeout_ms}ms with status {:?}: {stderr}",
            status.code()
        ),
    })
}

fn bun_release_target() -> Result<&'static str, ProbeRunnerError> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("macos", "aarch64") => Ok("darwin-aarch64"),
        ("macos", "x86_64") => Ok("darwin-x64"),
        ("linux", "aarch64") => Ok("linux-aarch64"),
        ("linux", "x86_64") => Ok("linux-x64"),
        (os, arch) => Err(ProbeRunnerError::UnsupportedAutoInstall {
            dependency: "bun",
            message: format!("unsupported platform {os}/{arch}"),
        }),
    }
}

fn deps_root(config: &CaptureConfig) -> PathBuf {
    let path = config
        .deps_dir
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(".style-scraper/deps"));
    if path.is_absolute() {
        path
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(path)
    }
}

fn bun_install_root(config: &CaptureConfig) -> PathBuf {
    deps_root(config)
        .join("bun")
        .join(format!("bun-v{BUN_VERSION}"))
}

fn local_bun_binary(config: &CaptureConfig) -> Result<PathBuf, ProbeRunnerError> {
    Ok(bun_install_root(config)
        .join(format!("bun-{}", bun_release_target()?))
        .join("bun"))
}

fn playwright_browsers_dir(config: &CaptureConfig) -> PathBuf {
    deps_root(config).join("playwright-browsers")
}

fn directory_has_entries(path: &Path) -> bool {
    path.read_dir()
        .map(|mut entries| entries.next().is_some())
        .unwrap_or(false)
}

#[derive(Debug)]
struct ProbeTempFiles {
    root: PathBuf,
    raw_path: PathBuf,
    stdout_path: PathBuf,
    stderr_path: PathBuf,
}

impl ProbeTempFiles {
    fn new(config: &CaptureConfig) -> Result<Self, ProbeRunnerError> {
        let base = config
            .artifact_dir
            .as_ref()
            .map(PathBuf::from)
            .unwrap_or_else(std::env::temp_dir);
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let root = base
            .join("tmp")
            .join(format!("style-scraper-{}-{nonce}", std::process::id()));
        fs::create_dir_all(&root)
            .map_err(|error| ProbeRunnerError::SpawnFailed(error.to_string()))?;
        Ok(Self {
            raw_path: root.join("raw-facts.json"),
            stdout_path: root.join("probe.stdout"),
            stderr_path: root.join("probe.stderr"),
            root,
        })
    }

    fn cleanup(&self, keep_artifacts: bool) {
        if !keep_artifacts {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
}

fn read_lossy(path: &Path) -> String {
    fs::read(path)
        .map(|bytes| String::from_utf8_lossy(&bytes).to_string())
        .unwrap_or_default()
}

fn is_playwright_missing_error(stderr: &str) -> bool {
    stderr.contains("Cannot find package 'playwright'")
        || stderr.contains("Cannot find module 'playwright'")
        || stderr.contains("Module not found")
}

fn is_browser_missing_error(stderr: &str) -> bool {
    stderr.contains("Executable doesn't exist")
        || stderr.contains("browserType.launch")
        || stderr.contains("playwright install")
}
