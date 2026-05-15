pub fn extract_px_values(input: &str) -> Vec<f64> {
    let mut values = Vec::new();
    let mut current = String::new();
    for ch in input.chars() {
        if ch.is_ascii_digit() || ch == '.' || ch == '-' {
            current.push(ch);
            continue;
        }
        if ch == 'p' || ch == 'x' {
            current.push(ch);
            if current.ends_with("px") {
                let numeric = current.trim_end_matches("px");
                if let Ok(value) = numeric.parse::<f64>() {
                    values.push(round_px(value));
                }
                current.clear();
            }
            continue;
        }
        current.clear();
    }
    values
}

pub fn first_px(input: &str) -> Option<f64> {
    extract_px_values(input).into_iter().next()
}

pub fn px_token_value(value: f64) -> String {
    let rounded = round_px(value);
    if (rounded.fract()).abs() < f64::EPSILON {
        format!("{}px", rounded as i64)
    } else {
        let formatted = format!("{rounded:.3}");
        format!(
            "{}px",
            formatted.trim_end_matches('0').trim_end_matches('.')
        )
    }
}

pub fn round_px(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}
