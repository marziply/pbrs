use super::{parse_fields, parse_prop, unwrap_block};
use crate::lexer::{Field, Kind};

fn indent(depth: i8) -> String {
  (0..depth).map(|_| "  ").collect()
}

pub fn from_field(field: Field, depth: i8) -> String {
  match field {
    Field::Block(block) => unwrap_block(block, depth),
    Field::Property(prop) => format!(
      "{}pub {}: {}",
      indent(depth),
      prop.name,
      parse_prop(prop.r#type)
    ),
    // Field::Rpc {
    //   name,
    //   params
    // } => {}
    _ => String::new()
  }
}

pub fn into_field(kind: Kind, depth: i8) -> String {
  String::new()
}

pub fn into_struct(
  identifier: String,
  fields: Vec<Field>,
  depth: i8
) -> String {
  format!(
    "{}struct {} {{\n{}\n{}}}",
    indent(depth),
    identifier,
    parse_fields(fields, depth + 1),
    indent(depth)
  )
}
