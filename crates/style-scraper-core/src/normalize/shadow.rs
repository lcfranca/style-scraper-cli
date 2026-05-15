pub fn normalize_shadow(input: &str) -> Option<String> {
    let value = input.trim();
    if value.is_empty() || value.eq_ignore_ascii_case("none") {
        None
    } else {
        Some(value.to_string())
    }
}
