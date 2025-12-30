use omnilang_core::lexer::Lexer;

#[test]
fn lexer_handles_number_with_unit_suffix() {
    let source = "RULE:\n- IF Distance < 1m THEN Stop";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("lexer should succeed");
    // Expect to find a Number token with numeric value 1.0 (unit suffix ignored)
    let has_number = tokens.iter().any(|t| matches!(t.token_type, omnilang_core::lexer::TokenType::Number(n) if (n - 1.0).abs() < 1e-6));
    assert!(
        has_number,
        "lexer should parse numeric literal with unit suffix"
    );
}
