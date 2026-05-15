use serde::Serialize;
use std::fs;
use std::io::{self, Write};

pub fn write_json<T: Serialize>(value: &T, output: &str, pretty: bool) -> anyhow::Result<()> {
    let bytes = if pretty {
        serde_json::to_vec_pretty(value)?
    } else {
        serde_json::to_vec(value)?
    };
    write_bytes(&bytes, output)
}

pub fn write_string(value: &str, output: &str) -> anyhow::Result<()> {
    write_bytes(value.as_bytes(), output)
}

fn write_bytes(bytes: &[u8], output: &str) -> anyhow::Result<()> {
    if output == "-" {
        let mut stdout = io::stdout().lock();
        stdout.write_all(bytes)?;
        stdout.write_all(b"\n")?;
    } else {
        fs::write(output, bytes)?;
    }
    Ok(())
}
