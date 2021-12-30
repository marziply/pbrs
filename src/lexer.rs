mod tokenise;

use std::cell::RefCell;
use std::error::Error;
use std::ops::Deref;
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

fn group_tokens<'a, 'b, T>(iter: &'b mut T) -> TokenChildren<'a>
where
  T: Iterator<Item = Rc<&'a str>>
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
      _ => node
        .tokens
        .borrow_mut()
        .push(token.deref())
    }
  }

  Some(node.groups)
}

fn identify_scalar(token: String) -> Scalar {
  match token.as_str() {
    "int32" => Scalar::Int32,
    "string" => Scalar::r#String,
    "bool" => Scalar::Bool,
    _ => panic!("Unidentified scalar")
  }
}

fn identify_fields<'a>(children: TokenChildren<'a>) -> Vec<Field> {
  children
    .unwrap_or_else(|| Vec::new())
    .iter()
    .cloned()
    // .map(|group| group.into_ref())
    .map(|TokenGroup(tokens, groups)| match tokens[0] {
      "message" | "service" => {
        let (identifier, kind) = identify_kind(tokens, groups);

        Field::Block(Block {
          identifier,
          kind
        })
      }
      "rpc" => Field::Rpc(Rpc {
        name: tokens[1].to_string(),
        params: (tokens[3].to_string(), tokens[7].to_string())
      }),
      val => Field::Property(Property {
        r#type: identify_scalar(val.to_string()),
        name: tokens[1].to_string(),
        value: tokens[3]
          .parse()
          .expect("Invalid value for field")
      })
    })
    .collect()
}

fn identify_kind<'a>(
  tokens: Vec<&'a str>,
  children: TokenChildren<'a>
) -> (Option<String>, Kind) {
  match tokens[0] {
    id @ ("service" | "message") => {
      let name = Some(tokens[1].to_string());
      let fields = identify_fields(children);

      match id {
        "service" => (name, Kind::Service(fields)),
        "message" => (name, Kind::Message(fields)),
        _ => unreachable!()
      }
    }
    "syntax" => (None, Kind::Syntax(tokens[3].to_string())),
    "package" => (None, Kind::Package(tokens[1].to_string())),
    _ => (None, Kind::Unknown)
  }
}

fn build_blocks<'a>(group: Vec<TokenGroup<'a>>) -> Vec<Block> {
  group
    .iter()
    .cloned()
    // .map(|group| group.into_ref())
    .map(|TokenGroup(tokens, children)| {
      let (identifier, kind) = identify_kind(tokens, children);

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
