use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

#[test]
#[ignore = "requires local Bun, Playwright browser, and an available localhost port"]
fn never_stable_fixture_captures_partial_instead_of_timing_out() {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace = manifest_dir
        .parent()
        .and_then(|path| path.parent())
        .expect("workspace root");
    let fixture_dir = workspace.join("fixtures").join("reconstructive");
    let port = "32123";

    let mut server = Command::new("python3")
        .arg("-m")
        .arg("http.server")
        .arg(port)
        .arg("--directory")
        .arg(&fixture_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("start fixture server");
    thread::sleep(Duration::from_millis(600));

    let binary = std::env::var("CARGO_BIN_EXE_style-scraper").unwrap_or_else(|_| {
        workspace
            .join("target/debug/style-scraper")
            .display()
            .to_string()
    });
    let output = Command::new(binary)
        .arg("extract")
        .arg("--url")
        .arg(format!("http://127.0.0.1:{port}/never-stable.html"))
        .arg("--wait")
        .arg("auto")
        .arg("--capture-on-timeout")
        .arg("--capture-timeout-ms")
        .arg("8000")
        .arg("--max-stability-wait-ms")
        .arg("1000")
        .arg("--format")
        .arg("json")
        .output()
        .expect("run style-scraper");

    let _ = server.kill();
    let _ = server.wait();

    assert!(
        output.status.success(),
        "style-scraper failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("strict JSON stdout");
    let warning_codes = value["diagnostics"]["warnings"]
        .as_array()
        .expect("warnings array")
        .iter()
        .filter_map(|warning| warning["code"].as_str())
        .collect::<Vec<_>>();
    assert!(warning_codes.contains(&"CAPTURE_PARTIAL_TIMEOUT"));
    assert!(value["evidence"]["observed_nodes"]
        .as_array()
        .is_some_and(|nodes| !nodes.is_empty()));
}
