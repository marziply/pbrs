mod tokenise;

use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

pub type TokenChildren<'a> = Option<Vec<TokenGroup<'a>>>;

#[derive(Clone)]
pub struct TokenGroup<'a>(pub Vec<&'a str>, pub TokenChildren<'a>);

// Protobuf "kinds" to represent each type of element available within the
// syntax
#[derive(Clone)]
pub enum Kind<'a> {
  Service(Vec<Field<'a>>),
  Message(Vec<Field<'a>>),
  Syntax(&'a str),
  Package(&'a str),
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
pub enum Field<'a> {
  Block(Block<'a>),
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
pub struct Block<'a> {
  pub tokens: Vec<&'a str>,
  pub identifier: Option<&'a str>,
  pub kind: Kind<'a>
}

fn group_tokens<'a, 'b, T>(iter: &'b mut T) -> TokenChildren<'a>
where
  T: Iterator<Item = Rc<&'a str>>
{
  // All sibling tokens of the current tree node
  let tokens: RefCell<Vec<&str>> = RefCell::new(Vec::new());
  let mut groups = Vec::new();
  // Drain tokens from tokens array into a new Block that can be pushed
  // into the blocks array along with siblings of the current tree node
  let mut push = |ch: TokenChildren<'a>| {
    let drained = tokens.borrow_mut().drain(..).collect();
    let group: TokenGroup<'a> = TokenGroup(drained, ch);

    groups.push(group);
  };

  // Continuously iterate over all sibling nodes within the source code,
  // descending into an iterative callback loop that results in a tree
  // of blocks as deep as the source code is
  while let Some(token) = iter.next() {
    match *token {
      ";" => push(None),
      "{" => push(group_tokens(iter)),
      "}" => break,
      _ => tokens.borrow_mut().push(*token)
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
    .map(|TokenGroup(tokens, groups)| match tokens[0] {
      "message" | "service" => {
        let (identifier, kind) = identify_kind(tokens.clone(), groups);

        Field::Block(Block {
          tokens,
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
) -> (Option<&'a str>, Kind<'a>) {
  match tokens[0] {
    id @ ("service" | "message") => {
      let name = Some(tokens[1]);
      let fields = identify_fields(children);

      match id {
        "service" => (name, Kind::Service(fields)),
        "message" => (name, Kind::Message(fields)),
        _ => unreachable!()
      }
    }
    "syntax" => (None, Kind::Syntax(tokens[3])),
    "package" => (None, Kind::Package(tokens[1])),
    _ => (None, Kind::Unknown)
  }
}

fn build_blocks<'a>(group: Vec<TokenGroup<'a>>) -> Vec<Block<'a>> {
  group
    .iter()
    .cloned()
    .map(|TokenGroup(tokens, children)| {
      let (identifier, kind) = identify_kind(tokens.clone(), children);

      Block {
        tokens,
        identifier,
        kind
      }
    })
    .collect()
}

pub fn translate<'a>(input: String) -> Result<Vec<Block<'a>>, Box<dyn Error>> {
  let stripped = tokenise::strip_comments(&input)?;
  let extracted = tokenise::extract_tokens(&stripped)?;
  let groups = group_tokens(&mut extracted.iter().map(|v| Rc::new(*v)));

  Ok(build_blocks(groups.unwrap()))
}
