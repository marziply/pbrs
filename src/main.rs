mod lexer;
mod parser;
mod tokeniser;

use std::env::args;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
  let path = args()
    .nth(1)
    .expect("Missing file path argument");
  let file = read_to_string(path)?;
  let tokens = tokeniser::translate(file)?;
  let blocks = lexer::translate(&tokens);
  let code = parser::translate(blocks);

  println!("{}", code);

  Ok(())
}
