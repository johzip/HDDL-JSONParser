use super::*;

impl<'a> Parser<'a> {
    pub fn parse_predicates(&'a self) -> Result<Vec<Predicate<'a>>, ParsingError<'a>> {
        let mut finished = false;
        let mut predicates = vec![];
        while !finished {
            match self.tokenizer.get_token()? {
                TokenType::Punctuator(PunctuationType::LParentheses) => {
                    let predicate = self.parse_predicate_definition()?;
                    predicates.push(predicate);
                }
                TokenType::Punctuator(PunctuationType::RParentheses) => {
                    finished = true;
                }
                token  => {
                    let error = SyntacticError {
                        expected: "predicate definition".to_string(),
                        found: token,
                        line_number: self.tokenizer.get_line_number(),
                    };
                    return Err(ParsingError::Syntactic(error));
                }
            }
        }
        Ok(predicates)
    }

    // parses a SINGLE predicate definition
    fn parse_predicate_definition(&'a self) -> Result<Predicate<'a>, ParsingError<'a>> {
        match self.tokenizer.get_token()? {
            TokenType::Identifier(predicate_name) => {
                return Ok(Predicate {
                    name: predicate_name,
                    variables: self.parse_args()?,
                })
            }
            token => {
                let error = SyntacticError {
                    expected: "a predicate name".to_string(),
                    found: token,
                    line_number: self.tokenizer.get_line_number(),
                };
                return Err(ParsingError::Syntactic(error));
            }
        }
    }
}
