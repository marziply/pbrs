mod lexer;
mod parser;

use std::env::args;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
  let path = args()
    .nth(1)
    .expect("Missing file path argument");
  let file = read_to_string(path)?;
  let blocks = lexer::translate(file)?;
  let code = parser::translate(blocks);

  println!("{}", code);

  Ok(())
}
