use omnilang_core::evaluator::evaluate_condition;
use serde_json::json;

#[test]
fn percent_is_converted_to_fraction() {
    let ctx = json!({ "RiskScore": 0.15 });

    assert!(evaluate_condition("RiskScore < 20%", &ctx));
    assert!(!evaluate_condition("RiskScore > 20%", &ctx));
}

#[test]
fn distance_units_convert_to_meters() {
    let ctx = json!({ "ObstacleDistance": 5.0 }); // meters

    assert!(evaluate_condition("ObstacleDistance < 1km", &ctx));
    assert!(evaluate_condition("ObstacleDistance > 300cm", &ctx));
}
