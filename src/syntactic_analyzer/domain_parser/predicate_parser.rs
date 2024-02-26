use std::collections::{HashMap, HashSet};


use super::*;

impl <'a> Parser <'a> {
    pub fn parse_predicates(&mut self) {
        match self.tokenizer.get_token() {
            Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) => {
                let predicate = self.parse_predicate_definition();
                self.symbol_table.add_predicate(predicate.name, predicate.args);
                self.parse_predicates();
            },
            Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) => {
                return;
            },
            _ => {
                // TODO: better error handling
                panic!("expected predicate definition, found ...")
            }
        }
    }

    // parses a SINGLE predicate definition
    fn parse_predicate_definition(&self) -> Predicate<'a> {
        if let Ok(Some(Token::Identifier(predicate_name))) = self.tokenizer.get_token() {
            let predicate_arguments = self.parse_list();
            Predicate {
                name: predicate_name,
                args: predicate_arguments
            }
        } else {
            // TODO: better error handling
            panic!("expected predicate name")
        }
    }
}

struct Predicate<'a> {
    name: &'a str,
    args: TypedList<'a>
}