use super::*;

#[cfg(test)]
mod lexer_test {
    use super::*;
    #[test]
    pub fn punctuation_recognition_test() {
        let program = String::from("-( \n) ").into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match lexer.get_token() {
            Ok(TokenType::Punctuator(PunctuationType::Dash)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Punctuator(PunctuationType::LParentheses)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Punctuator(PunctuationType::RParentheses)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::EOF) => {},
            _ => panic!("wrong token")
        }
    }

    #[test]
    pub fn ordering_relation_recognition_test() {
        let program = String::from("<=  \n> >= < \n").into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match lexer.get_token() {
            Ok(TokenType::Operator(OperationType::LessThanOrEqual)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Operator(OperationType::GreaterThan)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Operator(OperationType::GreaterThanOrEqual)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Operator(OperationType::LessThan)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::EOF) => {},
            _ => panic!("wrong token")
        }
    }

    #[test]
    pub fn logical_operator_recognition_test() {
        let program = String::from("and or oneof not exists forall imply\n").into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match lexer.get_token() {
            Ok(TokenType::Operator(OperationType::And)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Operator(OperationType::Or)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Operator(OperationType::Xor)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Operator(OperationType::Not)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Operator(OperationType::Exists)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Operator(OperationType::ForAll)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Operator(OperationType::Implication)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::EOF) => {},
            _ => panic!("wrong token")
        }
    }

    #[test]
    pub fn variable_recognition_test() {
        let program = String::from("?test_id ?pred-aa ").into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match lexer.get_token() {
            Ok(TokenType::Identifier(x)) => {
                assert_eq!(x, &String::from("test_id"))
            },
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Identifier(x)) => {
                assert_eq!(x, &String::from("pred-aa"))
            },
            _ => panic!("wrong token")
        }
    }

    #[test]
    pub fn keyword_recognition_test() {
        let program = String::from(
            "define domain problem :requirements :objects :types :constants\n
            :predicates :init :htn :action :parameters :method :precondition\n
            :effect :subtasks :tasks :ordered-tasks :ordered-subtasks :order\n
            :ordering :constraints\n"
        ).into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match lexer.get_token() {
            Ok(TokenType::Keyword(KeywordName::Define)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Keyword(KeywordName::Domain)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Keyword(KeywordName::Problem)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Keyword(KeywordName::Requirements)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Keyword(KeywordName::Objects)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Keyword(KeywordName::Types)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Keyword(KeywordName::Constants)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Keyword(KeywordName::Predicates)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Keyword(KeywordName::Init)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Keyword(KeywordName::HTN)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Keyword(KeywordName::Action)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Keyword(KeywordName::Parameters)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Keyword(KeywordName::Method)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Keyword(KeywordName::Precondition)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Keyword(KeywordName::Effect)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Keyword(KeywordName::Subtasks)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Keyword(KeywordName::Subtasks)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Keyword(KeywordName::OrderedSubtasks)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Keyword(KeywordName::OrderedSubtasks)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Keyword(KeywordName::Ordering)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Keyword(KeywordName::Ordering)) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Keyword(KeywordName::Constraints)) => {},
            _ => panic!("wrong token")
        }
    }

    #[test]
    pub fn identifier_recognition_test() {
        let program = String::from(
            "var123 var_3123 te23 v\n"
        ).into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match lexer.get_token() {
            Ok(TokenType::Identifier("var123")) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Identifier("var_3123")) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Identifier("te23")) => {},
            _ => panic!("wrong token")
        }
        match lexer.get_token() {
            Ok(TokenType::Identifier("v")) => {},
            _ => panic!("wrong token")
        }
    }

    #[test]
    pub fn lookahead_test() {
        let program = String::from(
            "(:method vDC_to_vPC_2
                :parameters (?d1 ?d2 - AbstractDevice ?t - SignalType ?p1 ?p2 - Port)
                :task (ValidateDeviceConnection ?d1 ?d2 ?t)
                :precondition (and
                        (isPartOf ?p1 ?d1)
                        (isPartOf ?p2 ?d2)
                        (isPlugDirection ?p1 out)
                        (isPlugDirection ?p2 both)
                        (isSignalSource ?p1 ?t)
                        (isSignalDestination ?p2 ?t)
                )
                :subtasks (and
                    (ValidatePortConnection ?p1 ?p2 ?t)
                )
            ) "
        ).into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        loop {
            let peek = lexer.lookahead();
            let actual = lexer.get_token();
            match actual {
                Ok(TokenType::EOF) => {break;}
                _ => {}
            }
            assert_eq!(peek.unwrap(), actual.unwrap());
        }
    }
}