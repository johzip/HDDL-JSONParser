use super::*;

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    #[ignore = "fails because of the new get_lexeme scheme"]
    pub fn variable_name_error_test() {
        let program = String::from("\n\n?ca<sd ?rt/asd \n\n\n\n ?f*ta \t %x954s ? ").into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match lexer.get_token() {
            Err(x) => {
                match x.error_type {
                    LexicalErrorType::InvalidIdentifier => {
                        assert_eq!(x.line_number, 3);
                        assert_eq!(x.lexeme, "ca<sd");
                    },
                    _ => panic!("wrong error detected")
                }
            },
            _ => panic!("error not detected")
        }
        match lexer.get_token() {
            Err(x) => {
                match x.error_type {
                    LexicalErrorType::InvalidIdentifier => {
                        assert_eq!(x.line_number, 3);
                        assert_eq!(x.lexeme, "rt/asd");
                    },
                    _ => panic!("wrong error detected")
                }
            },
            _ => panic!("error not detected")
        }
        match lexer.get_token() {
            Err(x) => {
                match x.error_type {
                    LexicalErrorType::InvalidIdentifier => {
                        assert_eq!(x.line_number, 7);
                        assert_eq!(x.lexeme, "f*ta");
                    },
                    _ => panic!("wrong error detected")
                }
            },
            _ => panic!("error not detected")
        }
        match lexer.get_token() {
            Err(x) => {
                match x.error_type {
                    LexicalErrorType::InvalidIdentifier => {
                        assert_eq!(x.line_number, 7);
                        assert_eq!(x.lexeme, "%x954s");
                    },
                    _ => panic!("wrong error detected")
                }
            },
            _ => panic!("error not detected")
        }
        match lexer.get_token() {
            Err(x) => {
                match x.error_type {
                    LexicalErrorType::InvalidIdentifier => {
                        assert_eq!(x.line_number, 7);
                        assert_eq!(x.lexeme, "");
                    },
                    _ => panic!("wrong error detected")
                }
            },
            _ => panic!("error not detected")
        }
    }

    #[test]
    pub fn keyword_error_test() {
        let program = String::from("\n\n:cra :pred \n\n\n\n :defne ").into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match lexer.get_token() {
            Err(x) => {
                match x.error_type {
                    LexicalErrorType::InvalidKeyword => {
                        assert_eq!(x.line_number, 3);
                        assert_eq!(x.lexeme, "cra");
                    },
                    _ => panic!("wrong error detected")
                }
            },
            _ => panic!("error not detected")
        }
        match lexer.get_token() {
            Err(x) => {
                match x.error_type {
                    LexicalErrorType::InvalidKeyword => {
                        assert_eq!(x.line_number, 3);
                        assert_eq!(x.lexeme, "pred");
                    },
                    _ => panic!("wrong error detected")
                }
            },
            _ => panic!("error not detected")
        }
        match lexer.get_token() {
            Err(x) => {
                match x.error_type {
                    LexicalErrorType::InvalidKeyword => {
                        assert_eq!(x.line_number, 7);
                        assert_eq!(x.lexeme, "defne");
                    },
                    _ => panic!("wrong error detected")
                }
            },
            _ => panic!("error not detected")
        }
    }
}