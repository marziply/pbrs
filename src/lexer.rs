mod identifier;
mod tokenise;

use identifier::Identifier;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

pub type TokenChildren<'a> = Option<Vec<TokenGroup<'a>>>;

#[derive(Clone)]
pub struct TokenGroup<'a>(pub Vec<&'a str>, pub TokenChildren<'a>);

// Protobuf "kinds" to represent each type of element available within the
// syntax
#[derive(Clone)]
pub enum Kind {
  Service(Vec<Field>),
  Message(Vec<Field>),
  Syntax(String),
  Package(String),
  Unknown
}

#[derive(Clone)]
pub struct Property {
  pub name: String,
  pub r#type: Scalar,
  pub value: i32
}

#[derive(Clone)]
pub struct Rpc {
  pub name: String,
  pub params: (String, String)
}

// Available field types within kind blocks, AKA anything enclosed in "{}",
// including other blocks as this is valid syntax in Protobuf
#[derive(Clone)]
pub enum Field {
  Block(Block),
  Property(Property),
  Rpc(Rpc)
}

// Basic scalar types available for fields within a block
#[derive(Clone)]
pub enum Scalar {
  Int32,
  Bool,
  r#String
}

// Any "block" of code, which can be either a simple expression or a scoped
// block of code that's wrapped with "{}"
#[derive(Clone)]
pub struct Block {
  // pub tokens: Vec<&'a str>,
  pub identifier: Option<String>,
  pub kind: Kind
}

#[derive(Default)]
struct Node<'a> {
  tokens: RefCell<Vec<&'a str>>,
  groups: Vec<TokenGroup<'a>>
}

impl<'a> Node<'a> {
  fn push(&mut self, children: TokenChildren<'a>) {
    // Drain tokens from tokens array into a new Block that can be pushed
    // into the blocks array along with siblings of the current tree node
    let drained = self
      .tokens
      .borrow_mut()
      .drain(..)
      .collect();
    let group: TokenGroup<'a> = TokenGroup(drained, children);

    self.groups.push(group);
  }
}

fn group_tokens<'inner, 'outer, T>(iter: &'outer mut T) -> TokenChildren<'inner>
where
  T: Iterator<Item = Rc<&'inner str>>
{
  // All sibling tokens of the current tree node
  let mut node = Node::default();

  // Continuously iterate over all sibling nodes within the source code,
  // descending into an iterative callback loop that results in a tree
  // of blocks as deep as the source code is
  while let Some(token) = iter.next() {
    match *token {
      ";" => node.push(None),
      "{" => node.push(group_tokens(iter)),
      "}" => break,
      _ => node.tokens.borrow_mut().push(*token)
    }
  }

  Some(node.groups)
}

fn build_blocks<'a>(group: Vec<TokenGroup<'a>>) -> Vec<Block> {
  group
    .iter()
    .cloned()
    .map(|TokenGroup(tokens, children)| {
      let (identifier, kind) = Identifier::create(tokens, children);

      Block {
        identifier,
        kind
      }
    })
    .collect()
}

pub fn translate<'a>(input: String) -> Result<Vec<Block>, Box<dyn Error>> {
  let stripped = tokenise::strip_comments(&input)?;
  let extracted = tokenise::extract_tokens(&stripped)?;
  let mut tokens = extracted
    .iter()
    .cloned()
    .map(|v| Rc::new(v));
  let groups = group_tokens(&mut tokens);
  let blocks = build_blocks(groups.unwrap());

  Ok(blocks)
}
