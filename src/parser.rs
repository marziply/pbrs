mod translate;

use self::translate::{from_field, into_struct};
use super::lexer::{Block, Field, Kind, Scalar};

fn parse_prop(scalar: Scalar) -> String {
  let result = match scalar {
    Scalar::Int32 => "i32",
    Scalar::Bool => "bool",
    Scalar::r#String => "string"
  };

  result.to_string()
}

fn parse_fields(fields: Vec<Field>, depth: u8) -> String {
  fields
    .iter()
    .cloned()
    .map(|v| from_field(v, depth))
    .collect::<Vec<String>>()
    .join(",\n")
}

fn unwrap_block(block: Block, depth: u8) -> String {
  let id = block
    .identifier
    .unwrap_or_else(|| String::new());

  match block.kind {
    Kind::Service(fields) => into_struct(id, fields, depth),
    Kind::Message(fields) => into_struct(id, fields, depth),
    _ => String::new()
  }
}

pub fn translate(blocks: Vec<Block>) -> String {
  blocks
    .iter()
    .cloned()
    .map(|v| unwrap_block(v, 0))
    .collect::<Vec<String>>()
    .join("\n\n")
}
