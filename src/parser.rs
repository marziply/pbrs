use fancy_regex::{Error as RegexError, Regex};
use std::cell::RefCell;
use std::error::Error;

type BlockOption<'a> = Option<Vec<Block<'a>>>;

#[derive(Debug)]
pub struct Block<'a>(Vec<&'a str>, BlockOption<'a>);

#[allow(dead_code)]
#[derive(Debug)]
pub enum Kind<'a> {
  Service(Block<'a>),
  Message(Block<'a>),
  Syntax(String),
  Package(String)
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Field<'a> {
  Property(Scalar),
  Rpc(String, String, String),
  Kind(Kind<'a>)
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Scalar {
  Int32,
  Bool,
  r#String
}

// pub fn identify_kind(input: &str) -> Kind {
//   match input {
//     "syntax" => Kind::Syntax,
//     "package" => Kind::Package,
//     "service" => Kind::Service,
//     "message" => Kind::Message,
//     _ => panic!("Invalid expression")
//   }
// }

pub fn strip_comments(raw_str: &str) -> Result<String, RegexError> {
  let re = Regex::new(r"//.*")?;
  let result = re.replace_all(raw_str, "").into_owned();

  Ok(result)
}

pub fn extract_tokens(raw_str: &str) -> Result<Vec<&str>, RegexError> {
  let re = Regex::new("[[:alnum:]]+|[[:punct:]]")?;
  let result = re
    .captures_iter(&raw_str)
    .flat_map(|v| -> Vec<&str> {
      v.unwrap()
        .iter()
        .map(|i| i.unwrap().as_str())
        .collect()
    })
    .collect();

  Ok(result)
}

struct Lexer<'a, T> {
  iter: &'a mut T
}

impl<'a, T> Lexer<'a, T>
where
  T: Iterator<Item = &'a str>
{
  pub fn group_tokens(&mut self) -> BlockOption<'a> {
    let tokens: RefCell<Vec<&str>> = RefCell::new(Vec::new());
    let mut blocks = Vec::new();
    let mut append_block = |ch: BlockOption<'a>| {
      blocks.push(Block(tokens.borrow_mut().drain(..).collect(), ch))
    };

    while let Some(token) = self.iter.next() {
      match token {
        ";" => append_block(None),
        "{" => append_block(self.group_tokens()),
        "}" => break,
        _ => tokens.borrow_mut().push(token)
      }
    }

    Some(blocks)
  }
}

pub fn parse(input: String) -> Result<String, Box<dyn Error>> {
  let stripped = strip_comments(&input)?;
  let extracted = extract_tokens(&stripped)?;
  let mut lexer = Lexer {
    iter: &mut extracted.iter().map(|v| v.to_owned())
  };
  let groups = lexer.group_tokens();

  println!("{:?}", groups.unwrap());

  Ok(String::from("foo"))
}
