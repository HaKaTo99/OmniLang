use omnilang_core::evaluator::evaluate_condition;
use serde_json::json;

#[test]
fn in_operator_checks_equality() {
    let ctx = json!({ "Mode": 2.0, "modes": [1,2,3] });
    assert!(evaluate_condition("Mode IN 2", &ctx));
    assert!(evaluate_condition("Mode IN modes", &ctx));
    assert!(!evaluate_condition("Mode IN 3", &ctx));
}

#[test]
fn missing_variables_do_not_mock_and_fail_safely() {
    let ctx = json!({});
    assert!(!evaluate_condition("MissingVar > 1", &ctx));
}

#[test]
fn boolean_and_or_chains_are_supported() {
    let ctx = json!({ "temperature": 40, "humidity": 80 });
    assert!(evaluate_condition("temperature > 30 AND humidity < 90", &ctx));
    assert!(!evaluate_condition("temperature > 30 AND humidity > 90", &ctx));
    assert!(evaluate_condition("temperature < 10 OR humidity < 90", &ctx));
}

#[test]
fn literal_array_membership_is_supported() {
    let ctx = json!({ "Mode": 2, "label": "b" });
    assert!(evaluate_condition("Mode IN [1,2,3]", &ctx));
    assert!(!evaluate_condition("Mode IN [4,5]", &ctx));
    assert!(evaluate_condition("label IN [\"a\",\"b\"]", &ctx));
}
