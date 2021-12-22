use fancy_regex::Regex;
use std::error::Error;

pub enum Block {
  Expression(Vec<String>),
  Scope(Vec<String>, Box<Block>)
}

pub enum Kind {
  Service(Field),
  Message(Vec<Field>),
  Syntax(String),
  Package(String)
}

pub enum Field {
  Property(Scalar),
  Rpc(String, String, String)
}

pub enum Scalar {
  Int32,
  Bool,
  r#String
}

impl Kind {}

// pub fn identify_kind(input: &str) -> Kind {
//   match input {
//     "syntax" => Kind::Syntax,
//     "package" => Kind::Package,
//     "service" => Kind::Service,
//     "message" => Kind::Message,
//     _ => panic!("Invalid expression")
//   }
// }

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
  let re = Regex::new(r"[[:alnum:]]+|[[:punct:]]")?;
  let tokens = extract_tokens(input.as_str());
  let result = tokens.join(" ");
  let matches: Vec<&str> = re
    .captures_iter(result.as_str())
    .flat_map(|v| -> Vec<&str> {
      v.unwrap()
        .iter()
        .map(|i| i.unwrap().as_str())
        .collect()
    })
    .collect();

  println!("{:?}", matches);
  // let mut token_iter = tokens.iter();
  // let mut kinds: Vec<Kind> = Vec::new();
  // let mut blocks: Vec<Block> = Vec::new();
  //
  // for token in tokens {}

  Ok(String::from("foo"))
}
