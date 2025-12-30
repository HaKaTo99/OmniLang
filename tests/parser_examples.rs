use omnilang_core::lexer::Lexer;
use omnilang_core::parser::Parser;

fn parse_file(path: &str) -> Result<(), String> {
    let source = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().map_err(|e| format!("{path}: {e}"))?;
    let mut parser = Parser::new(tokens);
    parser
        .parse_policy()
        .map_err(|e| format!("{path}: {e}"))?;
    Ok(())
}

#[test]
fn parse_examples_suite() {
    let files = [
        "examples/hello.omni",
        "examples/loop_demo.omni",
        "examples/policy.omni",
        "examples/units_and_loops.omni",
        "examples/nested_loops_units.omni",
        "examples/factory_safety.omni",
        "examples/drone_patrol.omni",
        "examples/hospital_policy.omni",
        "examples/edge_units_nested.omni",
        "examples/actions_loop_showcase.omni",
        "examples/ai_ethics_governance.omni",
        "examples/zero_trust_security.omni",
        "examples/anti_money_laundering.omni",
        "examples/green_data_center.omni",
        "examples/global_supply_chain.omni",
        "examples/smart_city_traffic.omni",
    ];

    let mut failures = Vec::new();
    for f in files {
        if let Err(e) = parse_file(f) {
            failures.push(e);
        }
    }

    if !failures.is_empty() {
        panic!("\n{}", failures.join("\n"));
    }
}
