pub fn normalize_font_family(input: &str) -> Option<String> {
    let first = input.split(',').next()?.trim();
    let family = first.trim_matches('"').trim_matches('\'').trim();
    if family.is_empty() {
        None
    } else {
        Some(family.to_string())
    }
}

pub fn normalize_font_weight(input: &str) -> Option<String> {
    let value = input.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}
