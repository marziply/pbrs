mod lexer;
mod parser;
mod tokenise;

use std::env::args;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
  let path = args()
    .nth(1)
    .expect("Missing file path argument");
  let file = read_to_string(path)?;
  let stripped = tokenise::strip_comments(file.as_str())?;
  let extracted = tokenise::extract_tokens(stripped.as_str())?;
  let blocks = lexer::translate(extracted);
  let code = parser::translate(blocks);

  println!("{}", code);

  Ok(())
}
