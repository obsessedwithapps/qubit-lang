use anyhow::{anyhow, Result};

use crate::lexer::{Lexer, TokenType};

#[derive(Debug)]
pub enum Op {
    Number(usize),
    Str(String),
    Function(String),

    // Gates and Bits
    Gate,
    Qubit,
    Cbit,

    // Control Flow
    If,
    Else,
    For,
    While,

    // Arithmetic
    Add,
    Sub,
    Mult,
    Div,
    Mod,

    // Misc
    OCBracket,                    // {
    CCBracket,                    // }
    OParen,                       // (
    CParen,                       // )
    Arguments(String, TokenType), // Function arguments
}

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            lexer: Lexer::new(),
        }
    }

    pub fn parse_token(&self, file: &str) -> Result<Vec<Op>> {
        let mut ops = Vec::<Op>::new();
        let tokens = self.lexer.lex_file(file)?;
        let mut iter = tokens.iter();

        while let Some(token) = iter.next() {
            match token.tok_type {
                TokenType::Word => match token.value.as_str() {
                    "if" => ops.push(Op::If),
                    "else" => ops.push(Op::Else),
                    "for" => ops.push(Op::For),
                    "while" => ops.push(Op::While),
                    "add" | "+" => ops.push(Op::Add),
                    "sub" | "-" => ops.push(Op::Sub),
                    "mult" | "*" => ops.push(Op::Mult),
                    "div" | "/" => ops.push(Op::Div),
                    "mod" | "%" => ops.push(Op::Mod),
                    "gate" => ops.push(Op::Gate),
                    "qbit" => ops.push(Op::Qubit),
                    "cbit" => ops.push(Op::Cbit),
                    "{" => ops.push(Op::OCBracket),
                    "}" => ops.push(Op::CCBracket),
                    "(" => ops.push(Op::OParen),
                    ")" => ops.push(Op::CParen),
                    "function" => {
                        let mut name = String::new();

                        if let Some(token) = iter.next() {
                            if token.tok_type != TokenType::Word {
                                eprintln!()
                            } else {
                                name = token.value.clone();
                            }
                        }

                        ops.push(Op::Function(name))
                    }
                    _ => {
                        return Err(anyhow!(
                            "{file} in line {row}: Unknown word {val:?}",
                            val = token.value,
                            row = token.row
                        ))
                    }
                },

                TokenType::Number => {
                    let val = token.value.parse::<usize>()?;
                    ops.push(Op::Number(val));
                }

                TokenType::Str => {
                    let str: String = token.value.clone();
                    ops.push(Op::Str(str));
                }
            }
        }

        Ok(ops)
    }
}
