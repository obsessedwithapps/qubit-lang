use std::io::Result;

pub struct Lexer;

impl Lexer {
    // Lexes an entire file 
    pub fn lex_file(file: &str) -> Result<Vec<String>> {
        let mut tokens: Vec<String> = Vec::new();
        let program = std::fs::read_to_string(file)?;
        let lines: Vec<&str> = program.lines().collect();

        for (row, line) in lines.iter().enumerate() {
            let mut lexed = Self::lex_line(line, row);
            tokens.append(&mut lexed);
        }

        Ok(tokens)
    }
    
    // Turns a line into a vector of strings or tokens
    pub fn lex_line(line: &str, row: usize) -> Vec<String> {
        let split = line.split("//").next().unwrap_or(line);
        let mut tokens: Vec<String> = Vec::new();
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
                            tokens.push(keyword);
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
                            tokens.push(number.to_owned());
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