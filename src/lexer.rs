mod tokenise;

use std::cell::RefCell;
use std::error::Error;

pub type TokenChildren = Option<Vec<TokenGroup>>;

#[derive(Debug)]
pub struct TokenGroup(pub Vec<String>, pub TokenChildren);

#[allow(dead_code)]
#[derive(Debug)]
pub enum Kind {
  Service(Vec<Field>),
  Message(Vec<Field>),
  Syntax(String),
  Package(String)
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Field {
  Block(Block),
  Property(Scalar),
  Rpc(String, Kind, Kind)
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Scalar {
  Int32,
  Bool,
  r#String
}

// Block of code represented as an enum of varying length token strings with
// an optional vector of another enum of itself that represents scoped
// children within the source code
#[derive(Debug)]
pub struct Block {
  pub tokens: Vec<String>,
  pub identifier: Option<String>,
  pub kind: Kind
}

// impl Block {
//   pub fn descriptor(&self) -> String {
//     self.0.first().unwrap().to_owned()
//   }
//
//   pub fn identifier(&self) -> String {
//     self.0.last().unwrap().to_owned()
//   }
// }

fn group_tokens<'a>(
  iter: &'a mut impl Iterator<Item = String>
) -> TokenChildren {
  // All sibling tokens of the current tree node
  let tokens: RefCell<Vec<String>> = RefCell::new(Vec::new());
  let mut groups = Vec::new();
  // Drain tokens from tokens array into a new Block that can be pushed
  // into the blocks array along with siblings of the current tree node
  let mut push = |ch: TokenChildren| {
    let drained = tokens.borrow_mut().drain(..).collect();
    let group = TokenGroup(drained, ch);

    groups.push(group);
  };

  // Continuously iterate over all sibling node within the source code,
  // descending into an iterative callback loop that results in a tree
  // of blocks as deep as the source code is
  while let Some(token) = iter.next() {
    match token.as_str() {
      ";" => push(None),
      "{" => push(group_tokens(iter)),
      "}" => break,
      _ => tokens.borrow_mut().push(token)
    }
  }

  Some(groups)
}

// pub fn identify_kind(block: Block) -> Kind {
//   match block.descriptor().as_str() {
//     "service" => Kind::Service(block),
//     "message" => Kind::Message(block),
//     "syntax" => Kind::Syntax(block.identifier()),
//     "package" => Kind::Package(block.identifier()),
//     _ => panic!("Invalid expression")
//   }
// }

pub fn blocks<'a>(input: String) -> Result<Vec<Block>, Box<dyn Error>> {
  let stripped = tokenise::strip_comments(&input)?;
  let extracted = tokenise::extract_tokens(&stripped)?;
  let mut iter = extracted.iter().map(|v| v.to_owned());
  let groups = group_tokens(&mut iter).unwrap();

  println!("{:?}", groups);

  Ok(Vec::new())
}
