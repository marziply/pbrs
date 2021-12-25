mod tokenise;

use std::cell::RefCell;
use std::error::Error;

type BlockOption = Option<Vec<Block>>;

#[derive(Debug)]
pub struct Block(Vec<String>, BlockOption);

struct Lexer<'a, T> {
  iter: &'a mut T
}

impl<'a, T> Lexer<'a, T>
where
  T: Iterator<Item = String>
{
  pub fn group_tokens(&mut self) -> BlockOption {
    let tokens: RefCell<Vec<String>> = RefCell::new(Vec::new());
    let mut blocks = Vec::new();
    let mut push = |ch: BlockOption| {
      let drained_tokens = tokens.borrow_mut().drain(..).collect();

      blocks.push(Block(drained_tokens, ch));
    };

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

pub fn parse<'a>(input: String) -> Result<Vec<Block>, Box<dyn Error>> {
  let stripped = tokenise::strip_comments(&input)?;
  let extracted = tokenise::extract_tokens(&stripped)?;
  let mut lexer = Lexer {
    iter: &mut extracted.iter().map(|v| v.to_owned())
  };
  let blocks: Vec<Block> = lexer.group_tokens().unwrap();

  Ok(blocks)
}
