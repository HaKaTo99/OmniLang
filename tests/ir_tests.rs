use omnilang_core::ast::{self, Actor, Constraint, Impact, Policy, Review, Rule, StandardRule, Trace};
use omnilang_core::ir::build_policy_ir;

#[test]
fn build_ir_includes_flat_rules_and_guards() {
    let policy = Policy {
        intent: Some("Test".to_string()),
        actors: vec![Actor { role: "Pilot".into(), primary: true }],
        context: None,
        assumptions: vec![],
        rules: vec![
            Rule::Standard(StandardRule { condition: "A > 1".into(), action: "Log".into() }),
            Rule::For(ast::ForLoop {
                iterator: "item".into(),
                collection: "items".into(),
                body: vec![Rule::Standard(StandardRule { condition: "x == 1".into(), action: "Act".into() })],
            }),
        ],
        constraints: vec![Constraint { kind: "Technical".into(), description: "None".into() }],
        impacts: vec![Impact { kind: "Benefit".into(), description: "Ok".into() }],
        traces: vec![Trace { kind: "Evidence".into(), link: "link".into() }],
        reviews: vec![Review { interval: "weekly".into(), criteria: "consistency".into() }],
    };

    let ir = build_policy_ir(&policy);

    // Tree rules preserved
    assert_eq!(ir.rules.len(), 2);
    // Flattened rules: standard + loop + nested standard => 3
    assert_eq!(ir.flat_rules.len(), 3);

    // Guard metadata present on loops in both tree and flat view
    let guard_tree = match &ir.rules[1] {
        omnilang_core::ir::RuleIR::For(f) => Some(f.guard.max_iterations),
        _ => None,
    };
    assert_eq!(guard_tree, Some(50));

    let guard_flat = match &ir.flat_rules[1] {
        omnilang_core::ir::RuleIR::For(f) => Some(f.guard.max_time_ms),
        _ => None,
    };
    assert_eq!(guard_flat, Some(1000));
}

#[test]
fn build_ir_handles_empty_policy() {
    let policy = Policy {
        intent: None,
        actors: vec![],
        context: None,
        assumptions: vec![],
        rules: vec![],
        constraints: vec![],
        impacts: vec![],
        traces: vec![],
        reviews: vec![],
    };

    let ir = build_policy_ir(&policy);
    assert!(ir.rules.is_empty());
    assert!(ir.flat_rules.is_empty());
}
