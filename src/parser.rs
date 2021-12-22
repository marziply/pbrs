use fancy_regex::{Error as RegError, Regex};
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

pub fn strip_comments(input: &str) -> Result<String, RegError> {
  let re = Regex::new(r"//.*")?;
  let result = re.replace_all(input, "").into_owned();

  Ok(result)
}

pub fn extract_tokens(input: &str) -> Result<Vec<&str>, RegError> {
  let re = Regex::new("[[:alnum:]]+|[[:punct:]]")?;
  let result = re
    .captures_iter(&input)
    .flat_map(|v| -> Vec<&str> {
      v.unwrap()
        .iter()
        .map(|i| i.unwrap().as_str())
        .collect()
    })
    .collect();

  Ok(result)
}

pub fn parse(input: String) -> Result<String, Box<dyn Error>> {
  let stripped = strip_comments(&input)?;
  let tokens = extract_tokens(&stripped)?;

  println!("{:?}", tokens);
  // let mut token_iter = tokens.iter();
  // let mut kinds: Vec<Kind> = Vec::new();
  // let mut blocks: Vec<Block> = Vec::new();
  //
  // for token in tokens {}

  Ok(String::from("foo"))
}
