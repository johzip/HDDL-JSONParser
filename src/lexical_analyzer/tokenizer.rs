
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
    pub fn lookahead(&self) -> Result<TokenType, LexicalError> {
        return self.parse(true);
    }

    // get current line number
    pub fn get_line_number(&self) -> u32 {
        return self.line_number.get();
    }

    pub fn get_token(&self) -> Result<TokenType, LexicalError> {
        return self.parse(false);
    }

    fn parse(&self, peek: bool) -> Result<TokenType, LexicalError> {
        self.skip_whitespaces();
        if self.cursor.get() == self.program.len() - 1 {
            return Ok(TokenType::EOF);
        }
        if let Some(char) = self.peek_next_char() {
            if !peek {
                self.cursor.set(self.cursor.get() + 1);
            }
            match char {
                // Punctuations
                '-' => Ok(TokenType::Punctuator(PunctuationType::Dash)),
                '(' => Ok(TokenType::Punctuator(PunctuationType::LParentheses)),
                ')' => Ok(TokenType::Punctuator(PunctuationType::RParentheses)),
                // Ordering Relations
                p @ ('<' | '>' | '=') => Ok(TokenType::Operator(self.ordering_type(&p, peek))),
                // Variables
                '?' => {
                    let mut init_cur_pos = self.cursor.get();
                    if peek {
                        init_cur_pos += 1;
                    }
                    let (var_name, new_cur_pos) = self.peek_lexeme(init_cur_pos)?;
                    if !peek {
                        self.cursor.set(new_cur_pos);
                    }
                    Ok(TokenType::Identifier(var_name))
                }
                // Keywords (Note that 2 keywords, namely "domain" and "problem", can start without ':' as well)
                ':' => {
                    let mut init_cur_pos = self.cursor.get();
                    if peek {
                        init_cur_pos += 1;
                    }
                    let (lexeme, new_cur_pos) = self.peek_lexeme(init_cur_pos)?;
                    if !peek {
                        self.cursor.set(new_cur_pos);
                    }
                    match lexeme {
                        // Requirements
                        "negative-preconditions" => Ok(TokenType::Requirement(
                            RequirementType::NegativePreconditions,
                        )),
                        "hierarchy" => Ok(TokenType::Requirement(RequirementType::Hierarchy)),
                        "equality" => Ok(TokenType::Requirement(RequirementType::Equality)),
                        "method-preconditions" => Ok(TokenType::Requirement(
                            RequirementType::MethodPreconditions,
                        )),
                        "typing" => Ok(TokenType::Requirement(RequirementType::TypedObjects)),
                        "universal-preconditions" => Ok(TokenType::Requirement(RequirementType::UniversalPreconditions)),
                        // Keywords
                        "requirements" => Ok(TokenType::Keyword(KeywordName::Requirements)),
                        "objects" => Ok(TokenType::Keyword(KeywordName::Objects)),
                        "types" => Ok(TokenType::Keyword(KeywordName::Types)),
                        "constants" => Ok(TokenType::Keyword(KeywordName::Constants)),
                        "predicates" => Ok(TokenType::Keyword(KeywordName::Predicates)),
                        "init" => Ok(TokenType::Keyword(KeywordName::Init)),
                        "htn" => Ok(TokenType::Keyword(KeywordName::HTN)),
                        "task" => Ok(TokenType::Keyword(KeywordName::Task)),
                        "action" => Ok(TokenType::Keyword(KeywordName::Action)),
                        "parameters" => Ok(TokenType::Keyword(KeywordName::Parameters)),
                        "method" => Ok(TokenType::Keyword(KeywordName::Method)),
                        "precondition" => Ok(TokenType::Keyword(KeywordName::Precondition)),
                        "effect" => Ok(TokenType::Keyword(KeywordName::Effect)),
                        "subtasks" | "tasks" => Ok(TokenType::Keyword(KeywordName::Subtasks)),
                        "ordered-subtasks" | "ordered-tasks" => {
                            Ok(TokenType::Keyword(KeywordName::OrderedSubtasks))
                        }
                        "ordering" | "order" => Ok(TokenType::Keyword(KeywordName::Ordering)),
                        "constraints" => Ok(TokenType::Keyword(KeywordName::Constraints)),
                        "goal" => Ok(TokenType::Keyword(KeywordName::Goal)),
                        "domain" => return Ok(TokenType::Keyword(KeywordName::Domain)),
                        "problem" => return Ok(TokenType::Keyword(KeywordName::Problem)),
                        _ => Err(LexicalError {
                            error_type: LexicalErrorType::InvalidKeyword,
                            line_number: self.line_number.get(),
                            lexeme: lexeme,
                        }),
                    }
                }
                // Comment
                ';' => {
                        let mut current = self.program[self.cursor.get()] as char;
                        while current != '\n' {
                            self.cursor.set(self.cursor.get() + 1);
                            current = self.program[self.cursor.get()] as char;
                        }
                        return self.get_token();
                }
                // Other
                _ => {
                    let mut init_cur_pos = self.cursor.get() - 1;
                    if peek {
                        init_cur_pos += 1;
                    }
                    let (lexeme, new_cur_pos) = self.peek_lexeme(init_cur_pos)?;
                    if !peek {
                        self.cursor.set(new_cur_pos);
                    }
                    match lexeme {
                        // Remaining Keywords
                        "define" => return Ok(TokenType::Keyword(KeywordName::Define)),
                        "domain" => return Ok(TokenType::Keyword(KeywordName::Domain)),
                        "problem" => return Ok(TokenType::Keyword(KeywordName::Problem)),
                        _ => {
                            // Logical Operators
                            match LexicalAnalyzer::is_logical_operator(&lexeme) {
                                Some(x) => return Ok(TokenType::Operator(x)),
                                // Identifier
                                None => {
                                    if lexeme.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
                                        return Ok(TokenType::Identifier(lexeme));
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
            Ok(TokenType::EOF)
        }
    }

    // get next lexeme and new cursor position (to commit peek)
    fn peek_lexeme(&self, init_cur_pos: usize) -> Result<(&str, usize), LexicalError> {
        let mut cursor_pos = init_cur_pos;
        let mut next_ch = self.program[cursor_pos] as char;
        let mut is_invalid = false;
        let mut is_valid_character = |c| match c {
            '_' | '-' => true,
            ')' | '(' => false,
            _ => {
                if LexicalAnalyzer::is_whitespace(&c) {
                    false
                } else {
                    if !c.is_alphanumeric() {
                        is_invalid = true;
                    }
                    true
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
        if is_invalid {
            return Err(LexicalError {
                error_type: LexicalErrorType::InvalidIdentifier,
                line_number: self.line_number.get(),
                lexeme: from_utf8(&self.program[init_cur_pos..cursor_pos]).unwrap()
            })
        } else {
            return Ok((from_utf8(&self.program[init_cur_pos..cursor_pos]).unwrap(), cursor_pos))
        }
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
            ' ' | '\t' | '\n' | '\r' => true,
            _ => false,
        }
    }
}
