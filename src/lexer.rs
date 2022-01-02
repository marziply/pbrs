mod identifier;

pub use identifier::{Block, Identifier, *};
use std::cell::RefCell;
use std::rc::Rc;

pub type TokenChildren<'a> = Option<Vec<TokenGroup<'a>>>;

#[derive(Clone)]
pub struct TokenGroup<'a>(pub Vec<&'a str>, pub TokenChildren<'a>);

#[derive(Clone)]
pub struct Graph<'a> {
  pub blocks: Vec<Block<'a>>,
  pub package: Option<String>
}

#[derive(Default)]
struct Node<'a> {
  tokens: RefCell<Vec<&'a str>>,
  groups: Vec<TokenGroup<'a>>
}

impl<'a> Node<'a> {
  fn push(&mut self, children: TokenChildren<'a>) {
    let group: TokenGroup<'a> = TokenGroup(self.drain(), children);

    self.groups.push(group);
  }

  fn drain(&mut self) -> Vec<&'a str> {
    // Drain tokens from tokens array into a new Block that can be pushed
    // into the blocks array along with siblings of the current tree node
    self
      .tokens
      .borrow_mut()
      .drain(..)
      .collect()
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

fn into_blocks<'a>(group: Vec<TokenGroup<'a>>) -> Vec<Block> {
  group
    .iter()
    .cloned()
    .map(|TokenGroup(tokens, children)| {
      let (identifier, kind) = Identifier::identify(tokens, children);

      Block {
        identifier,
        kind
      }
    })
    .collect()
}

pub fn translate<'a>(input: &'a Vec<String>) -> Vec<Block<'a>> {
  let mut tokens = input
    .iter()
    .map(|v| Rc::new(v.as_str()));
  let groups = group_tokens(&mut tokens);

  into_blocks(groups.unwrap())
}
