#[cfg(test)]
mod property_tests {
    use proptest::prelude::*;
    use omnilang_core::parser::Parser;
    use omnilang_core::lexer::Lexer;

    // Test 1: Parser should not panic on a wide range of string inputs.
    proptest! {
        #[test]
        fn test_parser_never_panics(
            source in prop::collection::vec(any::<char>(), 0..256)
                .prop_map(|chars| chars.into_iter().collect::<String>())
        ) {
            if let Ok(tokens) = Lexer::new(&source).tokenize() {
                // We don't care about the result of parsing, just that it doesn't panic.
                let _ = Parser::new(tokens).parse_policy();
            }
        }
    }

    // Test 2: Parser should correctly parse valid FOR loop syntax.
    proptest! {
        #[test]
        fn test_loop_syntax_validity(
            _items in prop::collection::vec("[a-z]+", 0..10),
            variable in "[a-z][a-z0-9_]*"
        ) {
            // A valid, simple FOR loop structure.
            let source = format!(
                r#"
INTENT: Loop test
ACTOR:
- Primary: Tester
CONTEXT:
- Domain: Test
- Lokasi: Lab
- Fase: Dev
RULE:
- FOR {variable} IN Participants {{
    - IF 1 > 0 THEN Approve_{variable}
}}
"#,
                variable = variable
            );
            
            let tokens_result = Lexer::new(&source).tokenize();
            prop_assert!(tokens_result.is_ok(), "Lexer should tokenize valid FOR loop ");
            
            if let Ok(tokens) = tokens_result {
                let ast_result = Parser::new(tokens).parse_policy();
                prop_assert!(ast_result.is_ok(), "Parser should parse valid FOR loop ");
            }
        }
    }
    
    /*
    // NOTE: This test is commented out as it relies on an idealized version of the runtime engine
    // that does not match the current implementation in `src/runtime.rs`.
    // The current `Runtime::execute_policy` returns a single `Decision` struct, not a Vec<Decision>.
    // The context loading and evaluation logic also differ.
    // This test should be revisited when the runtime is updated to match this design.
    #[test]
    fn test_loop_invariant() {
        use omnilang::ast::Policy;
        use omnilang::runtime::Runtime;
        use serde_json::json;
        
        proptest!(|(collection: Vec<String>)| {
            let source = format!(
                r#"RULE: - FOR item IN CONTEXT.items {{ RULE: - IF true THEN log(item) }}""#
            );
            
            let tokens = Lexer::new(&source).tokenize().unwrap();
            let policy = Parser::new(tokens).parse_policy().unwrap();
            
            // The current runtime does not support this form of context injection or evaluation.
            // let mut runtime = Runtime::new();
            // let context = ...;
            // let decisions = runtime.execute_policy(&policy);
            // prop_assert_eq!(decisions.len(), collection.len());
        });
    }
    */
}
