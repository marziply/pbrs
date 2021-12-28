use super::lexer::{Block, Field, Kind, Scalar};

fn indent(depth: u8) -> String {
  (0..depth).map(|_| "  ").collect()
}

fn parse_prop(scalar: Scalar) -> String {
  let result = match scalar {
    Scalar::Int32 => "i32",
    Scalar::Bool => "bool",
    Scalar::r#String => "string"
  };

  result.to_string()
}

fn parse_fields(fields: Vec<Field>, depth: u8) -> String {
  let mut result: Vec<String> = Vec::new();

  for field in fields {
    match field {
      Field::Block(block) => result.extend(unwrap_block(block, depth + 1)),
      Field::Property {
        name,
        r#type,
        // Not much can be done with value for now
        value: _
      } => {
        result.push(format!(
          "{}pub {}: {},",
          indent(depth),
          name,
          parse_prop(r#type)
        ));
      }
      // Field::Rpc {
      //   name,
      //   params
      // } => {}
      _ => {}
    }
  }

  result.join("\n")
}

fn unwrap_block(block: Block, depth: u8) -> Vec<String> {
  let mut result: Vec<String> = Vec::new();

  match block.kind {
    Kind::Service(fields) => {
      let struct_item = format!(
        "{}struct {} {{\n{}{}\n{}}}",
        indent(depth),
        block.identifier.unwrap(),
        indent(depth + 1),
        parse_fields(fields, depth),
        indent(depth)
      );

      result.push(struct_item);
    }
    Kind::Message(fields) => {
      let message_item = format!(
        "{}struct {} {{\n{}{}\n{}}}",
        indent(depth),
        block.identifier.unwrap(),
        indent(depth + 1),
        parse_fields(fields, depth),
        indent(depth)
      );

      result.push(message_item);
    }
    _ => {}
  }

  result
}

pub fn translate(blocks: Vec<Block>) -> String {
  let mut result: Vec<String> = Vec::new();

  for block in blocks {
    result.extend(unwrap_block(block, 0));
  }

  result.join("\n\n")
}
