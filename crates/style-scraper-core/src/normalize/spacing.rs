use super::units;

pub fn spacing_values(input: &str) -> Vec<String> {
    units::extract_px_values(input)
        .into_iter()
        .filter(|value| *value >= 0.0)
        .map(units::px_token_value)
        .collect()
}
