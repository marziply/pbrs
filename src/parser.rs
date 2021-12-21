use std::error::Error;

pub enum Kind {
  Service,
  Message
}

pub enum Meta {
  Syntax,
  Package
}

pub enum Scalar {
  Int32,
  Bool,
  r#String
}

pub fn parse(input: String) -> Result<String, Box<dyn Error>> {
  Ok(String::from("foo"))
}
