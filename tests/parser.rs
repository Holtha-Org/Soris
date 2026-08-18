#[cfg(test)]
mod tests {
    use soris::frontend::parser::Parser;
    use soris::frontend::lexer::Lexer;

    #[test]
    fn test_lexer_di_macro() {
        let mut lexer = Lexer::new("di!(\"hola\");");
        let tokens = lexer.tokenize().expect("Error al tokenizar");
        assert!(!tokens.is_empty());
    }

    #[test]
    fn test_lexer_mientras() {
        let mut lexer = Lexer::new("mientras (x < 10) { }");
        let tokens = lexer.tokenize().expect("Error al tokenizar");
        assert!(!tokens.is_empty());
    }

    #[test]
    fn test_lexer_para_en() {
        let mut lexer = Lexer::new("para i en 0..5 { }");
        let tokens = lexer.tokenize().expect("Error al tokenizar");
        assert!(!tokens.is_empty());
    }

    #[test]
    fn test_lexer_nuevos_tipos() {
        let mut lexer = Lexer::new("var x: ent8 = 5; var y: ent64s = 10;");
        let tokens = lexer.tokenize().expect("Error al tokenizar");
        assert!(!tokens.is_empty());
    }

    #[test]
    fn test_lexer_keywords_nuevos() {
        let mut lexer = Lexer::new("fn rasgo opt result alg nada err cad txt");
        let tokens = lexer.tokenize().expect("Error al tokenizar");
        assert!(!tokens.is_empty());
    }

    #[test]
    fn test_parser_di_macro() {
        let mut parser = Parser::new("di!(\"hola\");").expect("Error al crear parser");
        let stmts = parser.parse().expect("Error al parsear");
        assert!(!stmts.is_empty());
    }

    #[test]
    fn test_parser_mientras() {
        let mut parser = Parser::new("mientras (x < 10) { }").expect("Error al crear parser");
        let stmts = parser.parse().expect("Error al parsear");
        assert!(!stmts.is_empty());
    }

    #[test]
    fn test_parser_para_en() {
        let mut parser = Parser::new("para i en 0..5 { }").expect("Error al crear parser");
        let stmts = parser.parse().expect("Error al parsear");
        assert!(!stmts.is_empty());
    }

    #[test]
    fn test_parser_si_elsi_sino() {
        let código = r#"
si (x > 0) {
    di!("positivo");
} elsi (x == 0) {
    di!("cero");
} sino {
    di!("negativo");
}
"#;
        let mut parser = Parser::new(código).expect("Error al crear parser");
        let stmts = parser.parse().expect("Error al parsear");
        assert!(!stmts.is_empty());
    }

    #[test]
    fn test_parser_var_declaration() {
        let mut parser = Parser::new("var x = 10;").expect("Error al crear parser");
        let stmts = parser.parse().expect("Error al parsear");
        assert!(!stmts.is_empty());
    }
}
