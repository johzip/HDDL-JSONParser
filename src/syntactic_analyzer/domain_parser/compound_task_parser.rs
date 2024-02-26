use super::*;

impl <'a> Parser <'a> {
    pub fn parse_compound_task(&mut self) {
        if let Ok(Some(Token::Identifier(task_name))) = self.tokenizer.get_token() {
            if let Ok(Some(Token::Keyword(KeywordName::Parameters))) = self.tokenizer.get_token() {
                if let Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) = self.tokenizer.get_token() {
                    let parameters = self.parse_list();
                    self.syntax_tree.add_compound_task(task_name, parameters);
                } else {
                    // TODO: better error handling
                    panic!("mising '(' after parameters")
                }

            } else {
                // TODO: better error handling
                panic!("expected keyword :parameters")
            }
        } else {
            // TODO: better error handling
            panic!("expected compound task name")
        }
    }
}

struct CompoundTask <'a> {
    name: &'a str,
    parameters: TypedList<'a>
}