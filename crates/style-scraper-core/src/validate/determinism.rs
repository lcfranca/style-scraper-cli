use serde::Serialize;
use sha2::{Digest, Sha256};

pub fn hash_json<T: Serialize>(value: &T) -> anyhow::Result<String> {
    let json = serde_json::to_vec(value)?;
    Ok(hex_sha256(&json))
}

pub fn hex_sha256(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut output = String::with_capacity(digest.len() * 2);
    for byte in digest {
        output.push_str(&format!("{byte:02x}"));
    }
    output
}
