use std::{cell::Cell, str::{from_utf8, from_utf8_unchecked}};

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
        if let Some(char) = self.next_char() {
            match char {
                // Punctuations
                p @ ('-' | ':' | '(' | ')') => {
                    Some(Token::Punctuator(LexicalAnalyzer::punctation_type(&p)))
                },
                // Ordering Relations
                p @ ('<' | '>' | '=' ) => {
                    Some(Token::Operator(self.ordering_type(&p)))
                },
                // TODO: Add logical operations

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
                }
                // Default
                _ => None
            }
        } else {
            None
        }
       
    }

    fn next_char(&self) -> Option<char> {
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
        Some(current)
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