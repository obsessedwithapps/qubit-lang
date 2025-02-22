use anyhow::{anyhow, Ok, Result};

#[derive(Debug, Clone)]
pub struct Token {
    pub tok_type: TokenType,
    pub value: String,
    pub row: usize,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TokenType {
    Word,
    Str,
    Number,
}

#[derive(Debug, Clone)]
pub struct Lexer;

impl Lexer {
    pub fn new() -> Self {
        Lexer
    }

    fn lex_line(&self, file: &str, line: &str, row: usize) -> Result<Vec<Token>> {
        let split = line.split("//").next().unwrap_or(line);
        let chars = split.chars().collect::<Vec<char>>();
        let mut tokens = Vec::<Token>::new();
        let mut iter: std::iter::Peekable<std::iter::Enumerate<std::slice::Iter<'_, char>>> =
            chars.iter().enumerate().peekable();

        while let Some(&(_, &c)) = iter.peek() {
            match c {
                '+' | '*' | '/' | '%' | '.' | '&' | '|' | '$' | '@' | '-' | '=' | '(' | ')' => {
                    let value = c.to_string();

                    tokens.push(Token {
                        tok_type: TokenType::Word,
                        value: value,
                        row,
                    });
                    iter.next();
                }
                '!' | '<' | '>' => {
                    let mut expression = c.to_string();
                    iter.next();

                    if let Some(&(_, '=')) = iter.peek() {
                        expression.push('=');
                        iter.next();
                    }

                    tokens.push(Token {
                        tok_type: TokenType::Word,
                        value: expression,
                        row,
                    });
                }
                '"' => {
                    iter.next();
                    let mut value = String::new();
                    while let Some((_, &next)) = iter.next() {
                        if next == '"' {
                            tokens.push(Token {
                                tok_type: TokenType::Str,
                                value: value,
                                row,
                            });
                            break;
                        } else if !next.is_ascii() {
                            return Err(anyhow!(
                                "\x1b[31m[ERROR] {file} in line {row}: Non-ascii character encountered\x1b[0m"
                            ));
                        } else {
                            value.push(next);
                        }
                    }
                }
                _ if c.is_ascii_digit() => {
                    let mut value = String::new();
                    while let Some(&(_, &next_c)) = iter.peek() {
                        if next_c.is_ascii_digit() {
                            value.push(next_c);
                            iter.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token {
                        tok_type: TokenType::Number,
                        value: value,
                        row,
                    });
                }
                _ if c.is_ascii_alphanumeric() => {
                    let mut keyword = String::new();
                    while let Some(&(_, &next_c)) = iter.peek() {
                        if next_c.is_ascii_alphanumeric() || next_c == '_' {
                            keyword.push(next_c);
                            iter.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token {
                        tok_type: TokenType::Word,
                        value: keyword,
                        row,
                    });
                }
                ' ' => {
                    iter.next();
                }
                _ => {
                    return Err(anyhow!(
                        "\x1b[31m[ERROR] {file} line {row}: Invalid token {c:?}\n[BADCODE] {line}\x1b[0m",
                        row = row + 1
                    ));
                }
            }
        }

        Ok(tokens)
    }

    pub fn lex_file(&self, file: &str) -> Result<Vec<Token>> {
        let program = std::fs::read_to_string(file)?;
        let lines = program.lines().collect::<Vec<&str>>();
        let mut tokens: Vec<Token> = Vec::new();

        for (row, &line) in lines.iter().enumerate() {
            let mut lexed = self.lex_line(file, line, row + 1)?;
            tokens.append(&mut lexed);
        }

        Ok(tokens)
    }
}
