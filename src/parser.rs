use super::lexer::Block;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Kind {
  Service(Block),
  Message(Block),
  Syntax(String),
  Package(String)
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Field {
  Property(Scalar),
  Rpc(String, String, String),
  Kind(Kind)
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Scalar {
  Int32,
  Bool,
  r#String
}

// pub fn identify_kind(input: &str) -> Kind {
//   match input {
//     "syntax" => Kind::Syntax,
//     "package" => Kind::Package,
//     "service" => Kind::Service,
//     "message" => Kind::Message,
//     _ => panic!("Invalid expression")
//   }
// }
