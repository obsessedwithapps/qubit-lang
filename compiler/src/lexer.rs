use std::io::Result;

pub struct Token {
    value: String,
    row: usize
}

impl Token {
    fn new(value: String, row: usize) -> Self {
        Self { value, row }
    }    
}

pub struct Lexer;

impl Lexer {
    // Lexes an entire file 
    pub fn lex_file(file: &str) -> Result<Vec<Token>> {
        let mut tokens: Vec<Token> = Vec::new();
        let program = std::fs::read_to_string(file)?;
        let lines: Vec<&str> = program.lines().collect();

        for (row, &line) in lines.iter().enumerate() {
            let mut lexed = Self::lex_line(line, row);
            tokens.append(&mut lexed);
        }

        Ok(tokens)
    }
    
    // Turns a line into a vector of strings or tokens
    fn lex_line(line: &str, row: usize) -> Vec<Token> {
        let split = line.split("//").next().unwrap_or(line);
        let mut tokens: Vec<Token> = Vec::new();
        let chars = split.chars().collect::<Vec<char>>();
        let mut iter = chars.iter().peekable();

        while let Some(&c) = iter.next() {
            match c {
                ' ' => (),
                _ if c.is_alphanumeric() => {
                    let mut keyword = String::new();

                    while let Some(&&ch) = iter.peek() {
                        if ch.is_alphanumeric() {
                            keyword.push(ch);
                            iter.next();
                        } else {
                            tokens.push(Token::new(keyword, row));
                            break;
                        }
                    }
                }
                _ if c.is_numeric() => {
                    let mut number = String::new();

                    while let Some(&&digit) = iter.peek() {
                        if digit.is_numeric() {
                            number.push(digit);
                            iter.next();
                        } else {
                            tokens.push(Token::new(number, row));
                            break;
                        }
                    }
                }
                _ => eprintln!()
            }
        }

        todo!()
    }
}