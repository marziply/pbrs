mod parser;

use std::env::args;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
  let proto_path = args()
    .nth(1)
    .expect("Missing file path argument");
  let proto_file = read_to_string(proto_path)?;

  parser::parse(proto_file)?;

  Ok(())
}
