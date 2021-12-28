mod tokenise;

use std::cell::RefCell;
use std::error::Error;

pub type TokenChildren = Option<Vec<TokenGroup>>;

#[derive(Debug, Clone)]
pub struct TokenGroup(pub Vec<String>, pub TokenChildren);

// Protobuf "kinds" to represent each type of element available within the
// syntax
pub enum Kind {
  Service(Vec<Field>),
  Message(Vec<Field>),
  Syntax(String),
  Package(String),
  Unknown
}

// Available field types within kind blocks, AKA anything enclosed in "{}",
// including other blocks as this is valid syntax in Protobuf
pub enum Field {
  Block(Block),
  Property {
    name: String,
    r#type: Scalar,
    value: i32
  },
  Rpc {
    name: String,
    params: (String, String)
  }
}

// Basic scalar types available for fields within a block
pub enum Scalar {
  Int32,
  Bool,
  r#String
}

// Any "block" of code, which can be either a simple expression or a scoped
// block of code that's wrapped with "{}"
pub struct Block {
  pub tokens: Vec<String>,
  pub identifier: Option<String>,
  pub kind: Kind
}

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

fn identify_scalar(token: String) -> Scalar {
  match token.as_str() {
    "int32" => Scalar::Int32,
    "string" => Scalar::r#String,
    "bool" => Scalar::Bool,
    _ => panic!("Unidentified scalar")
  }
}

fn identify_fields(children: TokenChildren) -> Vec<Field> {
  children
    .unwrap_or_else(|| Vec::new())
    .iter()
    .cloned()
    .map(|TokenGroup(tokens, groups)| match tokens[0].as_str() {
      "message" | "service" => {
        let (identifier, kind) = identify_kind(tokens.clone(), groups);

        Field::Block(Block {
          tokens: tokens.clone(),
          identifier,
          kind
        })
      }
      "rpc" => Field::Rpc {
        name: tokens[1].clone(),
        params: (tokens[3].clone(), tokens[7].clone())
      },
      _ => Field::Property {
        r#type: identify_scalar(tokens[0].clone()),
        name: tokens[1].clone(),
        value: tokens[3]
          .parse()
          .expect("Invalid value for field")
      }
    })
    .collect()
}

fn identify_kind(
  tokens: Vec<String>,
  children: TokenChildren
) -> (Option<String>, Kind) {
  match tokens[0].as_str() {
    id @ ("service" | "message") => {
      let name = Some(tokens[1].clone());
      let fields = identify_fields(children);

      match id {
        "service" => (name, Kind::Service(fields)),
        "message" => (name, Kind::Message(fields)),
        _ => unreachable!()
      }
    }
    "syntax" => (None, Kind::Syntax(tokens[3].clone())),
    "package" => (None, Kind::Package(tokens[1].clone())),
    _ => (None, Kind::Unknown)
  }
}

fn build_blocks(group: Vec<TokenGroup>) -> Vec<Block> {
  group
    .iter()
    .cloned()
    .map(|TokenGroup(tokens, children)| {
      let (identifier, kind) = identify_kind(tokens.clone(), children);

      Block {
        tokens: tokens.clone(),
        identifier,
        kind
      }
    })
    .collect()
}

pub fn translate<'a>(input: String) -> Result<Vec<Block>, Box<dyn Error>> {
  let stripped = tokenise::strip_comments(&input)?;
  let extracted = tokenise::extract_tokens(&stripped)?;
  let mut iter = extracted.iter().map(|v| v.to_owned());
  let groups = group_tokens(&mut iter);

  Ok(build_blocks(groups.unwrap()))
}
