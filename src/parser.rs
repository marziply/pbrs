mod stringify;

use super::lexer::{Block, Field, Kind, Scalar};
use std::collections::HashMap;
use stringify::indent;

#[derive(Default)]
struct Parser<'a> {
  config: HashMap<&'a str, &'a str>,
  depth: u8
}

impl<'a> Parser<'a> {
  pub fn parse(&mut self, blocks: Vec<Block<'a>>) -> String {
    let result = blocks
      .iter()
      .cloned()
      .filter_map(|v| self.unwrap_block(v))
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

  fn unwrap_block(&mut self, block: Block<'a>) -> Option<String> {
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

  pub fn from_field(&mut self, field: Field<'a>) -> String {
    match field {
      Field::Block(block) => self
        .unwrap_block(block)
        .unwrap_or_default(),
      Field::Property(prop) => format!(
        "{}pub {}: {}",
        indent(self.depth),
        prop.name,
        self.parse_prop(prop.r#type)
      ),
      Field::Rpc(rpc) => format!(
        "{}fn {}(req: {}) -> {} {{\n{}}}",
        indent(self.depth),
        rpc.name,
        rpc.params.0,
        rpc.params.1,
        indent(self.depth)
      )
    }
  }

  fn parse_prop(&self, scalar: Scalar) -> String {
    let result = match scalar {
      Scalar::Int32 => "i32",
      Scalar::Bool => "bool",
      Scalar::r#String => "string"
    };

    result.to_string()
  }

  fn parse_fields(&mut self, fields: &Vec<Field<'a>>) -> String {
    fields
      .iter()
      .cloned()
      .map(|v| self.from_field(v))
      .collect::<Vec<String>>()
      .join(",\n")
  }

  fn indent(&mut self, callback: impl Fn(&mut Self) -> String) -> String {
    self.depth += 1;

    let result = callback(self);

    self.depth -= 1;

    result
  }

  fn into_trait(&mut self, identifier: &str, fields: Vec<Field<'a>>) -> String {
    self.indent(|s| {
      format!(
        "{}pub trait {} {{\n{}\n{}}}",
        indent(s.depth),
        identifier,
        s.parse_fields(&fields),
        indent(s.depth)
      )
    })
  }

  fn into_struct(
    &mut self,
    identifier: &str,
    fields: Vec<Field<'a>>
  ) -> String {
    self.indent(|s| {
      format!(
        "{}pub struct {} {{\n{}\n{}}}",
        indent(s.depth),
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
