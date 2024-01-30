use super::*;

#[cfg(test)]
mod lexer_test {
    use super::*;
    #[test]
    pub fn punctuation_recognition_test() {
        let program = String::from("-:( \n)").into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match lexer.get_token().unwrap() {
            Token::Punctuator(PunctuationType::Dash) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Punctuator(PunctuationType::Colon) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Punctuator(PunctuationType::LParentheses) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Punctuator(PunctuationType::RParentheses) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            None => {},
            _ => panic!("wrong token")
        }
    }

    #[test]
    pub fn operation_recognition_test() {
        let program = String::from("<=  \n> >= <").into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match lexer.get_token().unwrap() {
            Token::Operator(OperationType::LessThanOrEqual) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Operator(OperationType::GreaterThan) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Operator(OperationType::GreaterThanOrEqual) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Operator(OperationType::LessThan) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            None => {},
            _ => panic!("wrong token")
        }
    }
    #[test]
    pub fn identifier_recognition_test() {
        let program = String::from("?test_id ?pred-aa ?").into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match lexer.get_token().unwrap() {
            Token::Identifier(x) => {
                assert_eq!(x, &String::from("test_id"))
            },
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Identifier(x) => {
                assert_eq!(x, &String::from("pred-aa"))
            },
            _ => panic!("wrong token")
        }
    }
}