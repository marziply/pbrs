mod stringify;

use self::stringify::into_trait;

use super::lexer::{Block, Field, Kind, Scalar};
use stringify::{from_field, into_struct};

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

fn unwrap_block(block: Block, depth: u8) -> Option<String> {
  let id = block.identifier.unwrap_or_default();

  match block.kind {
    Kind::Service(fields) => Some(into_trait(id, fields, depth)),
    Kind::Message(fields) => Some(into_struct(id, fields, depth)),
    _ => None
  }
}

pub fn translate(blocks: Vec<Block>) -> String {
  blocks
    .iter()
    .cloned()
    .filter_map(|v| unwrap_block(v, 0))
    .collect::<Vec<String>>()
    .join("\n\n")
}
