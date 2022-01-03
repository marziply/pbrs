pub mod stringify;

use super::lexer::{Block, Field, Kind, Scalar};
use heck::ToSnakeCase;
use regex::RegexBuilder;
use std::collections::HashMap;
use stringify::indent;

impl From<Scalar> for String {
  fn from(value: Scalar) -> String {
    let result = match value {
      Scalar::Int32 => "i32",
      Scalar::Bool => "bool",
      Scalar::r#String => "String"
    };

    result.to_string()
  }
}

#[derive(Default)]
struct Parser<'a> {
  config: HashMap<&'a str, &'a str>,
  root: Vec<String>
}

impl<'a> Parser<'a> {
  pub fn parse(&mut self, blocks: Vec<Block<'a>>) -> String {
    let items = blocks
      .iter()
      .cloned()
      .filter_map(|v| self.parse_block(v))
      .collect::<Vec<String>>();

    self.root.extend(items);

    self.result(self.root.join("\n\n"))
  }

  fn result(&mut self, input: String) -> String {
    let re = RegexBuilder::new(r"^")
      .multi_line(true)
      .build()
      .unwrap();

    match self.config.get("package") {
      Some(name) => {
        format!(
          "pub mod {} {{\n{}\n}}",
          name,
          re.replace_all(input.as_str(), indent(1))
        )
      }
      None => self.root.join("\n\n")
    }
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
      Kind::Service(fields) => {
        self
          .root
          .push(format!("pub struct {}Client {{}}", id));

        Some(self.scope("trait", id, fields))
      }
      Kind::Message(fields) => Some(self.scope("struct", id, fields)),
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
      Field::Block(block) => {
        let id = block.identifier.unwrap_or_default();
        let struct_block = self
          .parse_block(block)
          .unwrap_or_default();

        self.root.push(struct_block);

        format!("{}pub {}: {}", indent(1), id.to_snake_case(), id)
      }
      Field::Property(prop) => {
        let r#type: String = prop.r#type.clone().into();

        format!("{}pub {}: {}", indent(1), prop.name, r#type)
      }
      Field::Rpc(rpc) => {
        format!(
          "{}fn {}(req: {}) -> {} {{\n{}\n{}}}",
          indent(1),
          rpc.name.to_snake_case(),
          rpc.params.0,
          rpc.params.1,
          format!("{}{}::default()", indent(2), rpc.params.1),
          indent(1)
        )
      }
    }
  }

  fn scope(&mut self, desc: &str, id: &str, fields: Vec<Field<'a>>) -> String {
    format!("pub {} {} {{\n{}\n}}", desc, id, self.parse_fields(&fields))
  }
}

pub fn translate(blocks: Vec<Block>) -> String {
  let mut parser = Parser::default();

  parser.parse(blocks)
}
