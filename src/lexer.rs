mod tokenise;

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
    let mut push = |ch: BlockOption<'a>| {
      let drained_tokens = tokens.borrow_mut().drain(..).collect();

      blocks.push(Block(drained_tokens, ch));
    };

    while let Some(token) = self.iter.next() {
      match token {
        ";" => push(None),
        "{" => push(self.group_tokens()),
        "}" => break,
        _ => tokens.borrow_mut().push(token)
      }
    }

    Some(blocks)
  }

  // pub fn match_token(&mut self, callback: impl FnMut(BlockOption)) {}
}

pub fn parse(input: String) -> Result<String, Box<dyn Error>> {
  let stripped = tokenise::strip_comments(&input)?;
  let extracted = tokenise::extract_tokens(&stripped)?;
  let mut lexer = Lexer {
    iter: &mut extracted.iter().map(|v| v.to_owned())
  };
  let groups = lexer.group_tokens();

  println!("{:?}", groups.unwrap());

  Ok(String::from("foo"))
}
