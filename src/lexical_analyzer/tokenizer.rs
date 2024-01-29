use std::cell::Cell;

use super::token_types::*;

pub struct LexicalAnalyzer {
    program: Vec<u8>,
    cursor: Cell<usize>
}

impl LexicalAnalyzer {
    pub fn new(program: Vec<u8>) -> LexicalAnalyzer {
        LexicalAnalyzer {
            program,
            cursor: Cell::new(0)
        }
    }
    pub fn get_token(&self) -> Option<Token> {
        if self.cursor.get() >= self.program.len() {
            return None
        }
        let mut current = self.program[self.cursor.get()] as char;
        while LexicalAnalyzer::is_whitespace(&current) {
            self.cursor.set(self.cursor.get() + 1);
            current = self.program[self.cursor.get()] as char;
            if self.cursor.get() == self.program.len() {
                return None
            }       
        }
        self.cursor.set(self.cursor.get() + 1);
        match current {
            p @ ('-' | ':' | '(' | ')') => {
                Some(Token::Punctuator(LexicalAnalyzer::punctation_type(&p)))
            },
            _ => None
        }
    }

    fn punctation_type(p: &char) -> PunctuationType {
        match p {
            '-' => PunctuationType::Dash,
            ':' => PunctuationType::Colon,
            '(' => PunctuationType::LParentheses,
            ')' => PunctuationType::RParentheses,
            _   => panic!("char {} is not a punctuator.", p) 
        }
    }

    fn is_whitespace(c: &char) -> bool {
        match c {
            ' ' | '\t' | '\n' => true,
            _ => false
        }
    }
}