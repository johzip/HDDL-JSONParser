use core::{panic};
use std::{cell::Cell, str::{from_utf8}};

use super::token_types::*;
use super::errors::*;

pub struct LexicalAnalyzer {
    program: Vec<u8>,
    cursor: Cell<usize>,
    line_number: Cell<u32>,
}

// TODO: parse an actual file as an integration test
impl LexicalAnalyzer {
    pub fn new(program: Vec<u8>) -> LexicalAnalyzer {
        LexicalAnalyzer {
            program,
            cursor: Cell::new(0),
            line_number: Cell::new(1)
        }
    }
    pub fn get_token(&self) -> Result<Option<Token>, LexicalError> {
        self.skip_whitespaces();
        if self.cursor.get() == self.program.len() - 1 {
            return Ok(None);
        }
        if let Some(char) = self.next_char() {
            match char {
                // Punctuations
                '-' => Ok(Some(Token::Punctuator(PunctuationType::Dash))),
                '(' => Ok(Some(Token::Punctuator(PunctuationType::LParentheses))),
                ')' => Ok(Some(Token::Punctuator(PunctuationType::RParentheses))),
                // Ordering Relations
                p @ ('<' | '>' | '=' ) => {
                    Ok(Some(Token::Operator(self.ordering_type(&p))))
                },
                // Variables
                '?' => {
                    let start = self.cursor.get().clone();
                    let mut next = self.program[start] as char;
                    if LexicalAnalyzer::is_whitespace(&next) {
                        return Err(LexicalError {
                            error_type: LexicalErrorType::InvalidIdentifier,
                            line_number: self.line_number.get(),
                            lexeme: ""
                        });
                    }
                    while !LexicalAnalyzer::is_whitespace(&next) {
                        self.cursor.set(self.cursor.get() + 1);
                        if self.cursor.get() == self.program.len() {
                            break;
                        }
                        next = self.program[self.cursor.get()] as char;
                        let is_dash = next == '_' || next == '-';
                        if !next.is_alphanumeric() && !is_dash && !LexicalAnalyzer::is_whitespace(&next){
                            let error = LexicalError {
                                error_type: LexicalErrorType::InvalidIdentifier,
                                line_number: self.line_number.get(),
                                lexeme: self.get_lexeme(start)
                            };
                            return Err(error)
                        }
                    }
                    let var_name = &self.program[start..self.cursor.get()]; 
                    let var_name = from_utf8(var_name).unwrap_or_default();
                    Ok(Some(Token::Identifier(var_name)))
                },
                // Keywords (Note that 2 keywords, namely "domain" and "problem", do not start with ':')
                ':' => {
                    let lexeme = self.get_lexeme(self.cursor.get());
                    match lexeme {
                        "define" => Ok(Some(Token::Keyword(KeywordName::Define))),
                        "domain" => Ok(Some(Token::Keyword(KeywordName::Domain))),
                        "problem" => Ok(Some(Token::Keyword(KeywordName::Problem))),
                        "requirements" => Ok(Some(Token::Keyword(KeywordName::Requirements))),
                        "objects" => Ok(Some(Token::Keyword(KeywordName::Objects))),
                        "types" => Ok(Some(Token::Keyword(KeywordName::Types))),
                        "constants" => Ok(Some(Token::Keyword(KeywordName::Constants))),
                        "predicates" => Ok(Some(Token::Keyword(KeywordName::Predicates))),
                        "init" => Ok(Some(Token::Keyword(KeywordName::Init))),
                        "htn" => Ok(Some(Token::Keyword(KeywordName::HTN))),
                        "action" => Ok(Some(Token::Keyword(KeywordName::Action))),
                        "parameters" => Ok(Some(Token::Keyword(KeywordName::Parameters))),
                        "method" => Ok(Some(Token::Keyword(KeywordName::Method))),
                        "precondition" => Ok(Some(Token::Keyword(KeywordName::Precondition))),
                        "effect" => Ok(Some(Token::Keyword(KeywordName::Effect))),
                        "subtasks" | "tasks" => Ok(Some(Token::Keyword(KeywordName::Subtasks))),
                        "ordered-subtasks" | "ordered-tasks" => Ok(Some(Token::Keyword(KeywordName::OrderedSubtasks))),
                        "ordering" | "order" => Ok(Some(Token::Keyword(KeywordName::Ordering))),
                        "constraints" => Ok(Some(Token::Keyword(KeywordName::Constraints))),
                        _ => Err(LexicalError {
                            error_type: LexicalErrorType::InvalidKeyword,
                            line_number: self.line_number.get(),
                            lexeme: lexeme
                        })
                    }
                },
                // Other
                _ => { 
                    let lexeme = self.get_lexeme(self.cursor.get() - 1);
                    match lexeme {
                        // Remaining Keywords
                        "domain" => return Ok(Some(Token::Keyword(KeywordName::Domain))),
                        "problem" => return Ok(Some(Token::Keyword(KeywordName::Problem))),
                        _ => {
                            // Logical Operators
                            match LexicalAnalyzer::is_logical_operator(&lexeme) {
                                Some(x) => return Ok(Some(Token::Operator(x))),
                                // Identifier
                                None => {
                                    if lexeme.chars().all(|c| c.is_alphanumeric() || c == '_') {
                                        return Ok(Some(Token::Identifier(lexeme)));
                                    } else {
                                        Err(LexicalError {
                                            error_type: LexicalErrorType::InvalidIdentifier,
                                            line_number: self.line_number.get(),
                                            lexeme: lexeme
                                        })
                                    }
                                }
                            }
                        }
                    }
                 }
            }
        } else {
            Ok(None)
        }
    }

    fn get_lexeme(&self, init_pos: usize) -> &str {
        let mut cursor_pos = init_pos;
        let mut next_ch = self.program[cursor_pos] as char;
        while !LexicalAnalyzer::is_whitespace(&next_ch){
            if cursor_pos < self.program.len() - 1 {
                cursor_pos += 1;
                next_ch = self.program[cursor_pos] as char;
            } else {
                break;
            }
        }
        self.cursor.set(cursor_pos);
        from_utf8(&self.program[init_pos..cursor_pos]).unwrap()
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
            if current == '\n' {
                self.line_number.set(self.line_number.get() + 1);
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

    fn is_punctuator(c: &char) -> bool {
        match c {
            '-' | ')' | '(' => true,
            _ => false
        }
    }
}