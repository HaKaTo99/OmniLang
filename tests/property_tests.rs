#[cfg(test)]
mod property_tests {
    use proptest::prelude::*;
    use omnilang::lexer::Lexer;
    use omnilang::parser::Parser;

    proptest! {
        #[test]
        fn parser_never_panics(source in "[\x20-\x7E]{0,200}") {
            // The lexer should not panic; we ignore the result.
            let _ = Lexer::new(&source).tokenize();
            // Parsing may fail, but must not panic.
            let _ = Parser::new(Lexer::new(&source).tokenize().unwrap_or_default()).parse_policy();
        }
    }
}
