use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub enum Token {
    // Single Characters
    LeftParen { line: usize },
    RightParen { line: usize },
    LeftBrace { line: usize },
    RightBrace { line: usize },
    Comma { line: usize },
    Dot { line: usize },
    Minus { line: usize },
    Plus { line: usize },
    Semicolon { line: usize },
    Slash { line: usize },
    Asterisk { line: usize },

    // Equality
    Bang { line: usize },
    BangEqual { line: usize },
    Equal { line: usize },
    EqualEqual { line: usize },
    Greater { line: usize },
    GreaterEqual { line: usize },
    Less { line: usize },
    LessEqual { line: usize },

    // Literal
    Identifier { line: usize, literal: String },
    String { line: usize, literal: String },
    Number { line: usize, literal: f64 },

    //Keyword
    And { line: usize },
    Class { line: usize },
    Else { line: usize },
    False { line: usize },
    Fun { line: usize },
    For { line: usize },
    If { line: usize },
    Nil { line: usize },
    Or { line: usize },
    Print { line: usize },
    Return { line: usize },
    Super { line: usize },
    This { line: usize },
    True { line: usize },
    Var { line: usize },
    While { line: usize },

    Eof { line: usize },

    Invalid { message: String, line: usize },
}

pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner { source }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        let mut line: usize = 0;

        let mut char_iter_peekable = self.source.chars().peekable();

        while let Some(character) = char_iter_peekable.next() {
            let token = match character {
                '(' => Some(Token::LeftParen { line }),
                ')' => Some(Token::RightParen { line }),
                '{' => Some(Token::RightBrace { line }),
                '}' => Some(Token::LeftBrace { line }),
                ',' => Some(Token::Comma { line }),
                '.' => Some(Token::Dot { line }),
                '-' => Some(Token::Minus { line }),
                '+' => Some(Token::Plus { line }),
                ';' => Some(Token::Semicolon { line }),
                '*' => Some(Token::Asterisk { line }),

                // Divide or comment
                '/' => match char_iter_peekable.next_if_eq(&'/') {
                    Some(_) => {
                        // It's a comment, skip to EOL
                        while char_iter_peekable.next_if(|&c| c != '\n').is_some() {}
                        None
                    }
                    None => Some(Token::Slash { line }),
                },

                // Equality and Conditionals
                '!' => match char_iter_peekable.next_if_eq(&'=') {
                    Some(_) => Some(Token::BangEqual { line }),
                    None => Some(Token::Bang { line }),
                },
                '=' => match char_iter_peekable.next_if_eq(&'=') {
                    Some(_) => Some(Token::EqualEqual { line }),
                    None => Some(Token::Equal { line }),
                },
                '<' => match char_iter_peekable.next_if_eq(&'=') {
                    Some(_) => Some(Token::LessEqual { line }),
                    None => Some(Token::Less { line }),
                },
                '>' => match char_iter_peekable.next_if_eq(&'=') {
                    Some(_) => Some(Token::GreaterEqual { line }),
                    None => Some(Token::Greater { line }),
                },

                '"' => {
                    let mut literal: String = String::new();

                    while let Some(&c) = char_iter_peekable.peek() {
                        if c != '"' {
                            literal.push(c);
                        } else {
                            break;
                        }
                        char_iter_peekable.next();
                    }

                    char_iter_peekable.next_if_eq(&'"');

                    Some(Token::String { literal, line })
                }

                // Numeric literals
                '0'..='9' => {
                    Scanner::number_parse(&mut char_iter_peekable, line);
                    None
                }
                // Ignore whitespace
                ' ' | '\r' | '\t' => None,

                '\n' => {
                    line += 1;
                    None
                }

                _ => Some(Token::Invalid {
                    message: format!("Unexpected character {} line {}", character, line),
                    line,
                }),
            };

            if let Some(token) = token {
                tokens.push(token);
            }
        }

        tokens
    }

    fn number_parse(char_iter_peekable: &mut Peekable<Chars>, line: usize) -> Vec<Token> {
        fn parse_number_chunk(char_iter_peekable: &mut Peekable<Chars>) -> String {
            let mut literal: String = String::new();

            while let Some(&c) = char_iter_peekable.peek() {
                if c.is_numeric() {
                    literal.push(c);
                } else {
                    break;
                }
                char_iter_peekable.next();
            }

            literal
        }

        let mut tokens = vec![];

        let mut literal: String = String::new();

        literal.push_str(&parse_number_chunk(char_iter_peekable));

        match char_iter_peekable.peek() {
            Some('.') => {}
            _ => tokens.push(Token::Number {
                literal: literal.parse().unwrap(),
                line,
            }),
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::*;

    #[test]
    fn string_and_comment() {
        let scanner = Scanner::new("\"asd\" // Ignored comment".to_string());

        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 1);

        assert!(matches!(tokens[0], Token::String { .. }));

        match &tokens[0] {
            Token::String { literal, .. } => {
                assert_eq!(literal, "asd");
            }
            _ => {
                unreachable!();
            }
        }
    }

    #[test]
    fn newline_after_comment() {
        let scanner = Scanner::new("// Ignored comment\n \"asd\"".to_string());

        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 1);

        assert!(matches!(tokens[0], Token::String { .. }));

        match &tokens[0] {
            Token::String { literal, .. } => {
                assert_eq!(literal, "asd");
            }
            _ => {
                unreachable!();
            }
        }
    }

    #[test]
    fn number_and_comment() {
        let scanner = Scanner::new("420.69 // Ignored comment".to_string());

        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 1);

        match &tokens[0] {
            Token::Number { literal, .. } => {
                assert!((*literal - 420.69).abs() < f64::EPSILON);
            }
            _ => {
                unreachable!();
            }
        }
    }
}
