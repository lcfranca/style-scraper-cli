use style_scraper_probe_runner::{BunProbeRunner, CaptureConfig, ProbeRunner, ProbeRunnerError};

#[test]
fn missing_bun_returns_structured_error_code() {
    let runner = BunProbeRunner::with_paths(
        "definitely-not-bun-style-scraper-test",
        ".",
        "probe/src/capture.ts",
    );
    let error = runner
        .capture(&CaptureConfig {
            url: "https://example.com".to_string(),
            auto_install_deps: false,
            ..CaptureConfig::default()
        })
        .unwrap_err();

    assert!(matches!(error, ProbeRunnerError::BunNotFound));
    assert_eq!(error.code(), "BUN_NOT_FOUND");
    assert_eq!(error.exit_code(), 3);
}
