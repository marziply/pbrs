mod tokenise;

use std::cell::RefCell;
use std::error::Error;

pub type TokenChildren = Option<Vec<TokenGroup>>;

#[derive(Debug, Clone)]
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

pub fn identify_fields(children: TokenChildren) -> Vec<Field> {
  children
    .unwrap_or_else(|| Vec::new())
    .iter()
    .cloned()
    .map(|TokenGroup(tokens, groups)| match tokens[0].as_str() {
      "rpc" => Field::Rpc {
        name: tokens[1].clone(),
        params: (tokens[3].clone(), tokens[7].clone())
      },
      "message" | "service" => {
        let (block_id, block_k) = identify_kind(tokens.clone(), groups);
        let block = Block {
          tokens: tokens.clone(),
          identifier: block_id,
          kind: block_k
        };

        Field::Block(block)
      }
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

fn scoped_kind(
  token: String,
  children: TokenChildren,
  callback: impl FnOnce(Vec<Field>) -> Kind
) -> (Option<String>, Kind) {
  (Some(token), callback(identify_fields(children)))
}

fn identify_kind(
  tokens: Vec<String>,
  children: TokenChildren
) -> (Option<String>, Kind) {
  match tokens[0].as_str() {
    "service" => scoped_kind(tokens[1].clone(), children, |v| Kind::Service(v)),
    "message" => scoped_kind(tokens[1].clone(), children, |v| Kind::Message(v)),
    "syntax" => (None, Kind::Syntax(tokens[3].clone())),
    "package" => (None, Kind::Package(tokens[1].clone())),
    _ => panic!("Invalid block expression")
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

pub fn blocks<'a>(input: String) -> Result<Vec<Block>, Box<dyn Error>> {
  let stripped = tokenise::strip_comments(&input)?;
  let extracted = tokenise::extract_tokens(&stripped)?;
  let mut iter = extracted.iter().map(|v| v.to_owned());
  let groups = group_tokens(&mut iter);

  Ok(build_blocks(groups.unwrap()))
}
