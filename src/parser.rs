use std::error::Error;

#[derive(Debug)]
pub enum Kind {
  Service,
  Message,
  Syntax,
  Package,
  Comment
}

enum Block {
  Expression(String),
  Scope(Box<Block>)
}

enum Scalar {
  Int32,
  Bool,
  r#String
}

impl Kind {}

pub fn identify_kind(input: &str) -> Kind {
  match input {
    "syntax" => Kind::Syntax,
    "package" => Kind::Package,
    "service" => Kind::Service,
    "message" => Kind::Message,
    _ => panic!("Invalid expression")
  }
}

pub fn extract_tokens(input: &str) -> Vec<&str> {
  let lines = input.lines();
  let mut result: Vec<&str> = Vec::new();

  for line in lines {
    if let Some((tokens, ..)) = line.split_once("//") {
      result.push(tokens);
    } else {
      result.push(line);
    }
  }

  result
    .iter()
    .flat_map(|v| v.split_whitespace())
    .collect()
}

pub fn parse(input: String) -> Result<String, Box<dyn Error>> {
  let tokens = extract_tokens(input.as_str());
  let blocks: Vec<Block> = Vec::new();

  // for token in tokens {
  //   let kind = identify_kind(token);
  // }

  println!("{:?}", tokens);

  Ok(String::from("foo"))
}
