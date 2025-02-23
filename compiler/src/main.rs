mod lexer;
mod parser;

use parser::Parser;

fn main() -> anyhow::Result<()> {
    let parser = Parser::new();
    let stuff = parser.parse_token("examples/example.qbit")?;
    println!("{stuff:?}");

    Ok(())
}
