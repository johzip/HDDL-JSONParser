use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn file_type_test() {
        let program = String::from("(:define (domain jajaja)) ").into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        let parser = Parser::new(&lexer);
        match parser.parse() {
            Ok(_) => {},
            _ => panic!("parsing error")
        }
        let program = String::from("(:define (problem jajaja2) (:domain blahblah)) ").into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        let parser = Parser::new(&lexer);
        match parser.parse() {
            Ok(_) => {},
            _ => panic!("parsing error")
        }
    }

    #[test]
    pub fn objects_list_test() {
        let program = String::from(
            "(:define (problem p1) (:domain bal) a b c - d s - f t))"
        ).into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match Parser::new(&lexer).parse() {
            Ok(symbols) => {
                assert_eq!(symbols.objects.contains("a"), true);
                assert_eq!(symbols.objects.contains("b"), true);
                assert_eq!(symbols.objects.contains("c"), true);
                assert_eq!(symbols.objects.contains("s"), true);
                assert_eq!(symbols.objects.contains("t"), true);
            },
            Err(_) => panic!("parsing errors")
        }
    }
}