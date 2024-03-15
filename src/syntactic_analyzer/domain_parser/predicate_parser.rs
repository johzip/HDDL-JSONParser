use super::*;

impl<'a> Parser<'a> {
    pub fn parse_predicates(&'a self) -> Result<Vec<Predicate<'a>>, SyntacticError<'a>> {
        let mut finished = false;
        let mut predicates = vec![];
        while !finished {
            match self.tokenizer.get_token() {
                Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) => {
                    let predicate = self.parse_predicate_definition()?;
                    predicates.push(predicate);
                }
                Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) => {
                    finished = true;
                }
                _ => {
                    // TODO: better error handling
                    panic!("expected predicate definition, found ...")
                }
            }
        }
        Ok(predicates)
    }

    // parses a SINGLE predicate definition
    fn parse_predicate_definition(&'a self) -> Result<Predicate<'a>, SyntacticError<'a>> {
        if let Ok(Some(Token::Identifier(predicate_name))) = self.tokenizer.get_token() {
            let predicate_arguments = self.parse_list()?;
            Ok(Predicate {
                name: predicate_name,
                variables: predicate_arguments,
            })
        } else {
            // TODO: better error handling
            panic!("expected predicate name")
        }
    }
}
