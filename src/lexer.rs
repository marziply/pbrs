mod tokenise;

use std::cell::RefCell;
use std::error::Error;

type BlockOption = Option<Vec<Block>>;

// Block of code represented as an enum of varying length token strings with
// an optional vector of another enum of itself that represents scoped
// children within the source code
#[derive(Debug)]
pub struct Block(Vec<String>, BlockOption);

struct Lexer<'a, T> {
  iter: &'a mut T
}

impl<'a, T> Lexer<'a, T>
where
  T: Iterator<Item = String>
{
  fn group_tokens(&mut self) -> BlockOption {
    // All sibling tokens of the current tree node
    let tokens: RefCell<Vec<String>> = RefCell::new(Vec::new());
    let mut blocks = Vec::new();
    // Drain tokens from tokens array into a new Block that can be pushed
    // into the blocks array along with siblings of the current tree node
    let mut push = |ch: BlockOption| {
      let drained_tokens = tokens.borrow_mut().drain(..).collect();

      blocks.push(Block(drained_tokens, ch));
    };

    // Continuously iterate over all sibling node within the source code,
    // descending into an iterative callback loop that results in a tree
    // of blocks as deep as the source code is
    while let Some(token) = self.iter.next() {
      match token.as_str() {
        ";" => push(None),
        "{" => push(self.group_tokens()),
        "}" => break,
        _ => tokens.borrow_mut().push(token)
      }
    }

    Some(blocks)
  }
}

pub fn blocks<'a>(input: String) -> Result<Vec<Block>, Box<dyn Error>> {
  let stripped = tokenise::strip_comments(&input)?;
  let extracted = tokenise::extract_tokens(&stripped)?;
  let mut lexer = Lexer {
    iter: &mut extracted.iter().map(|v| v.to_owned())
  };

  Ok(lexer.group_tokens().unwrap())
}
