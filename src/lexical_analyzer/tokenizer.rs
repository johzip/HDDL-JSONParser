
use std::{cell::Cell, str::from_utf8};

use super::*;

pub struct LexicalAnalyzer {
    program: Vec<u8>,
    cursor: Cell<usize>,
    line_number: Cell<u32>,
}

impl LexicalAnalyzer {
    pub fn new(program: Vec<u8>) -> LexicalAnalyzer {
        LexicalAnalyzer {
            program,
            cursor: Cell::new(0),
            line_number: Cell::new(1),
        }
    }
    // get the next token without advancing the cursor
    pub fn lookahead(&self) -> Result<Option<Token>, LexicalError> {
        return self.parse(true);
    }

    pub fn get_token(&self) -> Result<Option<Token>, LexicalError> {
        return self.parse(false);
    }

    fn parse(&self, peek: bool) -> Result<Option<Token>, LexicalError> {
        self.skip_whitespaces();
        if self.cursor.get() == self.program.len() - 1 {
            return Ok(None);
        }
        if let Some(char) = self.peek_next_char() {
            if !peek {
                self.cursor.set(self.cursor.get() + 1);
            }
            match char {
                // Punctuations
                '-' => Ok(Some(Token::Punctuator(PunctuationType::Dash))),
                '(' => Ok(Some(Token::Punctuator(PunctuationType::LParentheses))),
                ')' => Ok(Some(Token::Punctuator(PunctuationType::RParentheses))),
                // Ordering Relations
                p @ ('<' | '>' | '=') => Ok(Some(Token::Operator(self.ordering_type(&p, peek)))),
                // Variables
                '?' => {
                    let mut init_cur_pos = self.cursor.get();
                    if peek {
                        init_cur_pos += 1;
                    }
                    let (var_name, new_cur_pos) = self.peek_lexeme(init_cur_pos);
                    if !peek {
                        self.cursor.set(new_cur_pos);
                    }
                    Ok(Some(Token::Identifier(var_name)))
                }
                // Keywords (Note that 2 keywords, namely "domain" and "problem", can start without ':' as well)
                ':' => {
                    let mut init_cur_pos = self.cursor.get();
                    if peek {
                        init_cur_pos += 1;
                    }
                    let (lexeme, new_cur_pos) = self.peek_lexeme(init_cur_pos);
                    if !peek {
                        self.cursor.set(new_cur_pos);
                    }
                    match lexeme {
                        // Requirements
                        "negative-preconditions" => Ok(Some(Token::Requirement(
                            RequirementType::NegativePreconditions,
                        ))),
                        "hierarchy" => Ok(Some(Token::Requirement(RequirementType::Hierarchy))),
                        "method-preconditions" => Ok(Some(Token::Requirement(
                            RequirementType::MethodPreconditions,
                        ))),
                        "typing" => Ok(Some(Token::Requirement(RequirementType::TypedObjects))),
                        "universal-preconditions" => Ok(Some(Token::Requirement(RequirementType::UniversalPreconditions))),
                        // Keywords
                        "requirements" => Ok(Some(Token::Keyword(KeywordName::Requirements))),
                        "objects" => Ok(Some(Token::Keyword(KeywordName::Objects))),
                        "types" => Ok(Some(Token::Keyword(KeywordName::Types))),
                        "constants" => Ok(Some(Token::Keyword(KeywordName::Constants))),
                        "predicates" => Ok(Some(Token::Keyword(KeywordName::Predicates))),
                        "init" => Ok(Some(Token::Keyword(KeywordName::Init))),
                        "htn" => Ok(Some(Token::Keyword(KeywordName::HTN))),
                        "task" => Ok(Some(Token::Keyword(KeywordName::Task))),
                        "action" => Ok(Some(Token::Keyword(KeywordName::Action))),
                        "parameters" => Ok(Some(Token::Keyword(KeywordName::Parameters))),
                        "method" => Ok(Some(Token::Keyword(KeywordName::Method))),
                        "precondition" => Ok(Some(Token::Keyword(KeywordName::Precondition))),
                        "effect" => Ok(Some(Token::Keyword(KeywordName::Effect))),
                        "subtasks" | "tasks" => Ok(Some(Token::Keyword(KeywordName::Subtasks))),
                        "ordered-subtasks" | "ordered-tasks" => {
                            Ok(Some(Token::Keyword(KeywordName::OrderedSubtasks)))
                        }
                        "ordering" | "order" => Ok(Some(Token::Keyword(KeywordName::Ordering))),
                        "constraints" => Ok(Some(Token::Keyword(KeywordName::Constraints))),

                        _ => Err(LexicalError {
                            error_type: LexicalErrorType::InvalidKeyword,
                            line_number: self.line_number.get(),
                            lexeme: lexeme,
                        }),
                    }
                }
                // Other
                _ => {
                    let mut init_cur_pos = self.cursor.get() - 1;
                    if peek {
                        init_cur_pos += 1;
                    }
                    let (lexeme, new_cur_pos) = self.peek_lexeme(init_cur_pos);
                    if !peek {
                        self.cursor.set(new_cur_pos);
                    }
                    match lexeme {
                        // Remaining Keywords
                        "define" => return Ok(Some(Token::Keyword(KeywordName::Define))),
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
                                            lexeme: lexeme,
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

    // get next lexeme and new cursor position (to commit peek)
    fn peek_lexeme(&self, init_cur_pos: usize) -> (&str, usize) {
        let mut cursor_pos = init_cur_pos;
        let mut next_ch = self.program[cursor_pos] as char;
        let is_valid_character = |c| match c {
            '_' | '-' => true,
            ')' | '(' => false,
            _ => {
                if LexicalAnalyzer::is_whitespace(&c) {
                    false
                } else {
                    c.is_alphanumeric()
                }
            }
        };
        while is_valid_character(next_ch) {
            if cursor_pos < self.program.len() - 1 {
                cursor_pos += 1;
                next_ch = self.program[cursor_pos] as char;
            } else {
                break;
            }
        }
        (from_utf8(&self.program[init_cur_pos..cursor_pos]).unwrap(), cursor_pos)
    }

    fn peek_next_char(&self) -> Option<char> {
        if self.cursor.get() >= self.program.len() {
            return None;
        }
        let current = self.program[self.cursor.get()] as char;
        Some(current)
    }

    fn skip_whitespaces(&self) {
        let mut current = self.program[self.cursor.get()] as char;
        while LexicalAnalyzer::is_whitespace(&current) {
            if self.cursor.get() == self.program.len() - 1 {
                return;
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
            "and" => Some(OperationType::And),
            "or" => Some(OperationType::Or),
            "oneof" => Some(OperationType::Xor),
            "not" => Some(OperationType::Not),
            "forall" => Some(OperationType::ForAll),
            "exists" => Some(OperationType::Exists),
            "imply" => Some(OperationType::Implication),
            _ => None,
        }
    }

    fn ordering_type(&self, c: &char, peek: bool) -> OperationType {
        match c {
            '<' => {
                match self.peek_next_char() {
                    Some('=') => {
                        if !peek {
                            self.cursor.set(self.cursor.get() + 1);
                        }
                        OperationType::LessThanOrEqual
                    }
                    _ => OperationType::LessThan,
                }
            }
            '>' => {
                match self.peek_next_char() {
                    Some('=') => {
                        if !peek {
                            self.cursor.set(self.cursor.get() + 1);
                        }
                        OperationType::GreaterThanOrEqual
                    }
                    _ => OperationType::GreaterThan,
                }
            }
            '=' => OperationType::Equal,
            _ => {
                panic!("not an ordering relation");
            }
        }
    }

    fn is_whitespace(c: &char) -> bool {
        match c {
            ' ' | '\t' | '\n' => true,
            _ => false,
        }
    }
}
