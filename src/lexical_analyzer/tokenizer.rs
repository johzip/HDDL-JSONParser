use std::{cell::Cell, str::{from_utf8}};

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
        self.skip_whitespaces();
        if self.cursor.get() == self.program.len() - 1 {
            return None;
        }
        if let Some(char) = self.next_char() {
            match char {
                // Punctuations
                '-' => Some(Token::Punctuator(PunctuationType::Dash)),
                ':' => Some(Token::Punctuator(PunctuationType::Colon)),
                '(' => Some(Token::Punctuator(PunctuationType::LParentheses)),
                ')' => Some(Token::Punctuator(PunctuationType::RParentheses)),
                // Ordering Relations
                p @ ('<' | '>' | '=' ) => {
                    Some(Token::Operator(self.ordering_type(&p)))
                },
                // Variables
                '?' => {
                    let start = self.cursor.get().clone();
                    // TODO: include line number 
                    if start == self.program.len() {
                        panic!("variable name cannot be empty.")
                    }
                    let mut next = self.program[start] as char;
                    while !LexicalAnalyzer::is_whitespace(&next) {
                        self.cursor.set(self.cursor.get() + 1);
                        if self.cursor.get() == self.program.len() {
                            break;
                        }
                        next = self.program[self.cursor.get()] as char;
                        let is_dash = next != '_' || next != '-';
                        if !next.is_alphanumeric() && !is_dash {
                            // TODO: include line number
                            panic!("unrecognized variable name")
                        }
                    }
                    let var_name = &self.program[start..self.cursor.get()]; 
                    let var_name = from_utf8(var_name).unwrap_or_default();
                    Some(Token::Identifier(var_name))
                },
                // Default
                _ => { 
                    let init_cursor_pos = self.cursor.get() - 1;
                    let mut next_ch = self.program[self.cursor.get()] as char;
                    while !LexicalAnalyzer::is_whitespace(&next_ch) {
                        if self.cursor.get() < self.program.len() - 1 {
                            self.cursor.set(self.cursor.get() + 1);
                            next_ch = self.program[self.cursor.get()] as char;
                        } else {
                            break;
                        }
                    }
                    let lexeme = from_utf8(&self.program[init_cursor_pos..self.cursor.get()])
                                        .unwrap().to_lowercase();
                    match LexicalAnalyzer::is_logical_operator(&lexeme) {
                        Some(x) => {return Some(Token::Operator(x))},
                        None => {}
                    }
                    // TODO: make better error messages/handling
                    panic!("{lexeme} is not a valid token")
                 }
            }
        } else {
            None
        }
       
    }

    fn next_char(&self) -> Option<char> {
        if self.cursor.get() >= self.program.len() {
            return None
        }
        let current = self.program[self.cursor.get()] as char;
        self.cursor.set(self.cursor.get() + 1);
        Some(current)
    }

    fn skip_whitespaces(&self) {
        let mut current = self.program[self.cursor.get()] as char;
        while LexicalAnalyzer::is_whitespace(&current) {
            if self.cursor.get() == self.program.len() - 1 {
                return
            }
            self.cursor.set(self.cursor.get() + 1);
            current = self.program[self.cursor.get()] as char;
        }
    }

    fn is_logical_operator(word: &str) -> Option<OperationType> {
        match word {
            "and" => {Some(OperationType::And)},
            "or" => {Some(OperationType::Or)},
            "oneof" => {Some(OperationType::Xor)},
            "not" => {Some(OperationType::Not)},
            "forall" => {Some(OperationType::ForAll)},
            "exists" => {Some(OperationType::Exists)},
            "imply" => {Some(OperationType::Implication)},
            _ => None
        }
    }

    fn ordering_type(&self, c: &char) -> OperationType {
        match c {
            '<' => {
                if self.cursor.get() >= self.program.len() {
                    return OperationType::LessThan;
                }
                let next = self.program[self.cursor.get()] as char;
                match next {
                    '=' => {
                        self.cursor.set(self.cursor.get() + 1);
                        OperationType::LessThanOrEqual
                    },
                    _ => {
                        OperationType::LessThan
                    }
                }
            }
            '>' => {
                if self.cursor.get() >= self.program.len() {
                    return OperationType::GreaterThan
                }
                let next = self.program[self.cursor.get()] as char;
                match next {
                    '=' => {
                        self.cursor.set(self.cursor.get() + 1);
                        OperationType::GreaterThanOrEqual
                    },
                    _ => {
                        OperationType::GreaterThan
                    }
                }
            },
            '=' => {
                OperationType::Equal
            },
            _   => {
                panic!("not an ordering relation");
            }
        }
    }

    fn is_whitespace(c: &char) -> bool {
        match c {
            ' ' | '\t' | '\n' => true,
            _ => false
        }
    }
}