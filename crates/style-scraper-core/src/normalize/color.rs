pub fn normalize_css_color(input: &str) -> Option<String> {
    let value = input.trim().to_ascii_lowercase();
    if value.is_empty()
        || value == "transparent"
        || value == "rgba(0, 0, 0, 0)"
        || value == "rgba(0,0,0,0)"
    {
        return None;
    }

    if let Some(hex) = normalize_hex(&value) {
        return Some(hex);
    }

    if value.starts_with("rgb(") || value.starts_with("rgba(") {
        return normalize_rgb_function(&value);
    }

    None
}

fn normalize_hex(value: &str) -> Option<String> {
    let raw = value.strip_prefix('#')?;
    match raw.len() {
        3 => {
            let mut output = String::from("#");
            for ch in raw.chars() {
                if !ch.is_ascii_hexdigit() {
                    return None;
                }
                output.push(ch);
                output.push(ch);
            }
            Some(output)
        }
        6 => {
            if raw.chars().all(|ch| ch.is_ascii_hexdigit()) {
                Some(format!("#{raw}"))
            } else {
                None
            }
        }
        8 => {
            if raw.chars().all(|ch| ch.is_ascii_hexdigit()) {
                let alpha = u8::from_str_radix(&raw[6..8], 16).ok()?;
                if alpha == 0 {
                    None
                } else {
                    Some(format!("#{}", &raw[..6]))
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

fn normalize_rgb_function(value: &str) -> Option<String> {
    let start = value.find('(')?;
    let end = value.rfind(')')?;
    let inner = &value[start + 1..end];
    let normalized = inner.replace([',', '/'], " ");
    let parts: Vec<&str> = normalized
        .split_whitespace()
        .filter(|part| !part.is_empty())
        .collect();
    if parts.len() < 3 {
        return None;
    }

    let r = parse_channel(parts[0])?;
    let g = parse_channel(parts[1])?;
    let b = parse_channel(parts[2])?;
    let alpha = parts.get(3).and_then(parse_alpha);
    if matches!(alpha, Some(a) if a <= 0.0) {
        return None;
    }

    Some(format!("#{r:02x}{g:02x}{b:02x}"))
}

fn parse_channel(input: &str) -> Option<u8> {
    if let Some(percent) = input.strip_suffix('%') {
        let value = percent.parse::<f64>().ok()?;
        return Some(((value.clamp(0.0, 100.0) / 100.0) * 255.0).round() as u8);
    }
    let value = input.parse::<f64>().ok()?;
    Some(value.clamp(0.0, 255.0).round() as u8)
}

fn parse_alpha(input: &&str) -> Option<f64> {
    if let Some(percent) = input.strip_suffix('%') {
        let value = percent.parse::<f64>().ok()?;
        return Some(value.clamp(0.0, 100.0) / 100.0);
    }
    input.parse::<f64>().ok().map(|value| value.clamp(0.0, 1.0))
}

#[cfg(test)]
mod tests {
    use super::normalize_css_color;

    #[test]
    fn normalizes_rgb_colors() {
        assert_eq!(
            normalize_css_color("rgb(37, 99, 235)").as_deref(),
            Some("#2563eb")
        );
        assert_eq!(normalize_css_color("rgba(0, 0, 0, 0)"), None);
    }
}
