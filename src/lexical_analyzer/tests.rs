use super::*;

#[cfg(test)]
mod lexer_test {
    use super::*;
    #[test]
    pub fn punctuation_recognition_test() {
        let program = String::from("-( \n) ").into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match lexer.get_token().unwrap() {
            Token::Punctuator(PunctuationType::Dash) => {},
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
    pub fn ordering_relation_recognition_test() {
        let program = String::from("<=  \n> >= < \n").into_bytes();
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
    pub fn logical_operator_recognition_test() {
        let program = String::from("and or oneof not exists forall imply\n").into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match lexer.get_token().unwrap() {
            Token::Operator(OperationType::And) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Operator(OperationType::Or) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Operator(OperationType::Xor) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Operator(OperationType::Not) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Operator(OperationType::Exists) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Operator(OperationType::ForAll) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Operator(OperationType::Implication) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            None => {},
            _ => panic!("wrong token")
        }
    }

    #[test]
    pub fn variable_recognition_test() {
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

    #[test]
    pub fn keyword_recognition_test() {
        let program = String::from(
            ":define domain problem :requirements :objects :types :constants\n
            :predicates :init :htn :action :parameters :method :precondition\n
            :effect :subtasks :tasks :ordered-tasks :ordered-subtasks :order\n
            :ordering :constraints\n"
        ).into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match lexer.get_token().unwrap() {
            Token::Keyword(KeywordName::Define) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Keyword(KeywordName::Domain) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Keyword(KeywordName::Problem) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Keyword(KeywordName::Requirements) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Keyword(KeywordName::Objects) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Keyword(KeywordName::Types) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Keyword(KeywordName::Constants) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Keyword(KeywordName::Predicates) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Keyword(KeywordName::Init) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Keyword(KeywordName::HTN) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Keyword(KeywordName::Action) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Keyword(KeywordName::Parameters) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Keyword(KeywordName::Method) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Keyword(KeywordName::Precondition) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Keyword(KeywordName::Effect) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Keyword(KeywordName::Subtasks) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Keyword(KeywordName::Subtasks) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Keyword(KeywordName::OrderedSubtasks) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Keyword(KeywordName::OrderedSubtasks) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Keyword(KeywordName::Ordering) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Keyword(KeywordName::Ordering) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Keyword(KeywordName::Constraints) => {},
            _ => panic!("wrong token")
        }
    }

    #[test]
    pub fn identifier_recognition_test() {
        let program = String::from(
            "var123 var_3123 te23 v\n"
        ).into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match lexer.get_token().unwrap() {
            Token::Identifier("var123") => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Identifier("var_3123") => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Identifier("te23") => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token().unwrap() {
            Token::Identifier("v") => {},
            _ => panic!("wrong token")
        }
    }
}