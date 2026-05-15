use std::fs;
use std::path::Path;

pub fn write_json_file(path: &Path, bytes: &[u8]) -> anyhow::Result<()> {
    fs::write(path, bytes)?;
    Ok(())
}
