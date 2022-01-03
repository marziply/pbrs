mod stringify;

use super::lexer::{Block, Field, Kind, Scalar};
use std::collections::HashMap;
use stringify::indent;

trait ItemParser<S = String> {
  type Input;

  fn parse(&mut self, item: Self::Input) -> S;
}

#[derive(Default)]
struct Parser<'a> {
  config: HashMap<&'a str, &'a str>,
  depth: u8
}

impl<'a> ItemParser<Option<String>> for Parser<'a> {
  type Input = Block<'a>;

  fn parse(&mut self, item: Self::Input) -> Option<String> {
    let id = item.identifier.unwrap_or_default();

    match item.kind {
      Kind::Service(fields) => Some(self.into_trait(id, fields)),
      Kind::Message(fields) => Some(self.into_struct(id, fields)),
      Kind::Package(name) => {
        self.config.insert("package", name);

        None
      }
      Kind::Syntax(syn) => {
        self.config.insert("syntax", syn);

        None
      }
      _ => None
    }
  }
}

impl<'a> Parser<'a> {
  pub fn parse(&mut self, blocks: Vec<Block<'a>>) -> String {
    let result = blocks
      .iter()
      .cloned()
      .filter_map(|v| self.parse_block(v))
      .collect::<Vec<String>>()
      .join("\n\n");

    self.parse_config(result)
  }

  fn parse_config(&self, result: String) -> String {
    self
      .config
      .iter()
      .fold(result, |acc, (key, value)| match *key {
        "package" => format!("pub mod {} {{\n{}\n}}", value, acc),
        "syntax" => acc,
        _ => acc
      })
  }

  fn parse_fields(&mut self, fields: &Vec<Field<'a>>) -> String {
    fields
      .iter()
      .cloned()
      .map(|v| self.from_field(v))
      .collect::<Vec<String>>()
      .join(",\n")
  }

  fn parse_block(&mut self, block: Block<'a>) -> Option<String> {
    let id = block.identifier.unwrap_or_default();

    match block.kind {
      Kind::Service(fields) => Some(self.into_trait(id, fields)),
      Kind::Message(fields) => Some(self.into_struct(id, fields)),
      Kind::Package(name) => {
        self.config.insert("package", name);

        None
      }
      Kind::Syntax(syn) => {
        self.config.insert("syntax", syn);

        None
      }
      _ => None
    }
  }

  fn from_field(&mut self, field: Field<'a>) -> String {
    match field {
      Field::Block(block) => self
        .parse_block(block)
        .unwrap_or_default(),
      Field::Property(prop) => self.indent(|s| {
        format!(
          "{}pub {}: {}",
          indent(s.depth),
          prop.name,
          s.from_scalar(&prop.r#type)
        )
      }),
      Field::Rpc(rpc) => self.indent(|s| {
        format!(
          "{}fn {}(req: {}) -> {} {{\n{}}}",
          indent(s.depth),
          rpc.name,
          rpc.params.0,
          rpc.params.1,
          indent(s.depth)
        )
      })
    }
  }

  fn from_scalar(&self, scalar: &Scalar) -> String {
    let result = match scalar {
      Scalar::Int32 => "i32",
      Scalar::Bool => "bool",
      Scalar::r#String => "String"
    };

    result.to_string()
  }

  fn indent(&mut self, callback: impl Fn(&mut Self) -> String) -> String {
    self.depth += 1;

    let result = callback(self);

    self.depth -= 1;

    result
  }

  fn into_trait(&mut self, id: &str, fields: Vec<Field<'a>>) -> String {
    self.scope("trait", id, fields)
  }

  fn into_struct(&mut self, id: &str, fields: Vec<Field<'a>>) -> String {
    self.scope("struct", id, fields)
  }

  fn scope(
    &mut self,
    descriptor: &str,
    identifier: &str,
    fields: Vec<Field<'a>>
  ) -> String {
    self.indent(|s| {
      format!(
        "{}pub {} {} {{\n{}\n{}}}",
        indent(s.depth),
        descriptor,
        identifier,
        s.parse_fields(&fields),
        indent(s.depth)
      )
    })
  }
}

pub fn translate(blocks: Vec<Block>) -> String {
  let mut parser = Parser::default();

  parser.parse(blocks)
}
