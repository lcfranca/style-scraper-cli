use style_scraper_core::model::raw::RawFacts;
use style_scraper_core::{analyze_raw_facts, AnalysisConfig};

#[test]
fn analyzing_same_raw_facts_is_deterministic() {
    let raw: RawFacts =
        serde_json::from_str(include_str!("../../../tests/golden/basic.raw-facts.json")).unwrap();
    let left = analyze_raw_facts(raw.clone(), AnalysisConfig::default()).unwrap();
    let right = analyze_raw_facts(raw, AnalysisConfig::default()).unwrap();

    let left_json = serde_json::to_string(&left).unwrap();
    let right_json = serde_json::to_string(&right).unwrap();

    assert_eq!(left_json, right_json);
    assert!(left.tokens.color.to_string().contains("#2563eb"));
    assert!(left
        .morphology
        .atoms
        .iter()
        .any(|atom| atom.kind == "button"));
    assert!(left
        .morphology
        .organisms
        .iter()
        .any(|organism| organism.kind == "header"));
}
