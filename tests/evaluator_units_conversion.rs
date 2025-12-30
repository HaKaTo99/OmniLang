use omnilang_core::evaluator::evaluate_condition;
use serde_json::json;

#[test]
fn km_equals_meters_after_normalization() {
    let data = json!({ "dist": "1km" });
    assert!(evaluate_condition("dist == 1000m", &data));
    assert!(evaluate_condition("dist >= 500m", &data));
    assert!(evaluate_condition("dist < 2km", &data));
}

#[test]
fn cm_and_meters_compare_consistently() {
    let data = json!({ "gap": "250cm" });
    assert!(evaluate_condition("gap == 2.5m", &data));
    assert!(evaluate_condition("gap > 200cm", &data));
    assert!(evaluate_condition("gap < 3m", &data));
}

#[test]
fn percent_as_fraction_compares_consistently() {
    let data = json!({ "ratio": "25%" });
    assert!(evaluate_condition("ratio == 0.25", &data));
    assert!(evaluate_condition("ratio < 0.5", &data));
    assert!(!evaluate_condition("ratio > 0.3", &data));
}

#[test]
fn milliseconds_and_seconds_compare_consistently() {
    let data = json!({ "latency": "1500ms" });
    assert!(evaluate_condition("latency == 1.5s", &data));
    assert!(evaluate_condition("latency < 2s", &data));
}
