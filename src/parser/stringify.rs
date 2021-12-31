use super::{parse_fields, parse_prop, unwrap_block};
use crate::lexer::Field;

fn indent(depth: u8) -> String {
  (0..depth).map(|_| "  ").collect()
}

pub fn from_field(field: Field, depth: u8) -> String {
  match field {
    Field::Block(block) => unwrap_block(block, depth).unwrap_or_default(),
    Field::Property(prop) => format!(
      "{}pub {}: {}",
      indent(depth),
      prop.name,
      parse_prop(prop.r#type)
    ),
    Field::Rpc(rpc) => format!(
      "{}fn {}(req: {}) -> {} {{\n{}}}",
      indent(depth),
      rpc.name,
      rpc.params.0,
      rpc.params.1,
      indent(depth)
    )
  }
}

pub fn into_trait(identifier: &str, fields: Vec<Field>, depth: u8) -> String {
  format!(
    "{}pub trait {} {{\n{}\n{}}}",
    indent(depth),
    identifier,
    parse_fields(fields, depth + 1),
    indent(depth)
  )
}

pub fn into_struct(identifier: &str, fields: Vec<Field>, depth: u8) -> String {
  format!(
    "{}pub struct {} {{\n{}\n{}}}",
    indent(depth),
    identifier,
    parse_fields(fields, depth + 1),
    indent(depth)
  )
}

pub fn into_mod(identifier: &str, fields: Vec<Field>, depth: u8) -> String {
  format!(
    "{}pub mod {} {{\n{}\n{}}}",
    indent(depth),
    identifier,
    parse_fields(fields, depth + 1),
    indent(depth)
  )
}
