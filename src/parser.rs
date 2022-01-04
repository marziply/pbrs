use super::lexer::{Block, Field, Kind, Scalar};
use heck::ToSnakeCase;
use regex::RegexBuilder;
use std::collections::HashMap;

pub fn indent(depth: u8) -> String {
  (0..depth).map(|_| "  ").collect()
}

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
    // Collect and parse all blocks of code into an array of String
    let items = blocks
      .iter()
      .cloned()
      .filter_map(|v| self.parse_block(v))
      .collect::<Vec<String>>();
    // Join the blocks onto the root collection of structs so all nested
    // structs are placed at the top of the rendered output
    let total = self
      .root
      .iter()
      .cloned()
      .chain(items)
      .collect::<Vec<String>>()
      .join("\n\n");

    self.result(total)
  }

  fn result(&mut self, input: String) -> String {
    // Replacer for adding indentation to each line
    let re = RegexBuilder::new(r"^")
      .multi_line(true)
      .build()
      .unwrap();

    // If the package has been defined, wrap the result in a mod block
    match self.config.get("package") {
      Some(name) => {
        format!(
          "pub mod {} {{\n{}\n}}",
          name,
          re.replace_all(&input, indent(1))
        )
      }
      None => input
    }
  }

  fn parse_block(&mut self, block: Block<'a>) -> Option<String> {
    let id = block.identifier.unwrap_or_default();

    match block.kind {
      Kind::Message(fields) => Some(self.format_block("struct", id, fields)),
      Kind::Service(fields) => {
        // While Protobuf supports nested message structures, Rust isn't so
        // forgiving - nested messages need to be moved to the root depth
        // where they can be referred to as types in child messages
        self
          .root
          .push(format!("pub struct {}Client {{}}", id));

        Some(self.format_block("trait", id, fields))
      }
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

  fn format_field(&mut self, field: Field<'a>) -> String {
    match field {
      Field::Block(block) => {
        let id = block.identifier.unwrap_or_default();
        let struct_block = self
          .parse_block(block)
          .unwrap_or_default();

        // Push the generated struct into the root collection so it can be
        // rendered at the top of the output
        self.root.push(struct_block);

        self.format_property(id.to_snake_case(), id.to_string())
      }
      Field::Property(prop) => {
        let r#type: String = prop.r#type.clone().into();

        self.format_property(prop.name.to_string(), r#type)
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

  fn format_property(&self, name: String, id: String) -> String {
    format!("{}pub {}: {}", indent(1), name, id)
  }

  fn format_block(
    &mut self,
    desc: &str,
    id: &str,
    fields: Vec<Field<'a>>
  ) -> String {
    let result = fields
      .iter()
      .cloned()
      .map(|v| self.format_field(v))
      .collect::<Vec<String>>()
      .join(",\n");

    format!("pub {} {} {{\n{}\n}}", desc, id, result)
  }
}

pub fn translate(blocks: Vec<Block>) -> String {
  let mut parser = Parser::default();

  parser.parse(blocks)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::lexer::{Property, Rpc};

  fn create_message<'a>() -> Vec<Block<'a>> {
    let fields = vec![Field::Property(Property {
      r#type: Scalar::Int32,
      name: "bar",
      value: 1
    })];

    vec![Block {
      identifier: Some("Foo"),
      kind: Kind::Message(fields)
    }]
  }

  #[test]
  fn translate_struct() {
    let input = create_message();
    let result = translate(input);

    assert_eq!(result, "pub struct Foo {\n  pub bar: i32\n}");
  }

  #[test]
  fn translate_trait() {
    let fields = vec![Field::Rpc(Rpc {
      name: "Foo",
      params: ("Request", "Response")
    })];
    let input = vec![Block {
      identifier: Some("Bar"),
      kind: Kind::Service(fields)
    }];
    let result = translate(input);

    assert_eq!(
      result,
      "pub struct BarClient {}\n\npub trait Bar {\n  fn foo(req: Request) -> \
       Response {\n    Response::default()\n  }\n}"
    );
  }

  #[test]
  fn wrap_package() {
    let mut input = create_message();

    input.push(Block {
      identifier: None,
      kind: Kind::Package("foobar")
    });

    let result = translate(input);

    assert_eq!(
      result,
      "pub mod foobar {\n  pub struct Foo {\n    pub bar: i32\n  }\n}"
    );
  }
}
