use super::TokenChildren;

// Protobuf "kinds" to represent each type of element available within the
// syntax
#[derive(Clone, PartialEq, Debug)]
pub enum Kind<'a> {
  Service(Vec<Field<'a>>),
  Message(Vec<Field<'a>>),
  Package(&'a str),
  Syntax(&'a str),
  Unknown
}

// Available field types within kind blocks, AKA anything enclosed in "{}",
// including other blocks as this is valid syntax in Protobuf
#[derive(Clone, PartialEq, Debug)]
pub enum Field<'a> {
  Block(Block<'a>),
  Property(Property<'a>),
  Rpc(Rpc<'a>)
}

// Basic scalar types available for fields within a block
#[derive(Clone, PartialEq, Debug)]
pub enum Scalar {
  Int32,
  Bool,
  r#String
}

#[derive(Clone, PartialEq, Debug)]
pub struct Property<'a> {
  pub r#type: Scalar,
  pub name: &'a str,
  pub value: i32
}

#[derive(Clone, PartialEq, Debug)]
pub struct Rpc<'a> {
  pub name: &'a str,
  pub params: (&'a str, &'a str)
}

// Any "block" of code, which can be either a simple expression or a scoped
// block of code that's wrapped with "{}"
#[derive(Clone, PartialEq, Debug)]
pub struct Block<'a> {
  pub identifier: Option<&'a str>,
  pub kind: Kind<'a>
}

pub struct Identifier<'a> {
  pub tokens: Vec<&'a str>,
  pub children: TokenChildren<'a>
}

impl<'a> From<Identifier<'a>> for (Option<&'a str>, Kind<'a>) {
  fn from(value: Identifier<'a>) -> Self {
    value.kind()
  }
}

impl<'a> From<Identifier<'a>> for Field<'a> {
  fn from(value: Identifier<'a>) -> Self {
    value.field()
  }
}

impl<'a> Identifier<'a> {
  pub fn identify<T>(tokens: Vec<&'a str>, children: TokenChildren<'a>) -> T
  where
    T: From<Identifier<'a>>
  {
    let this = Self {
      tokens,
      children
    };

    this.into()
  }

  fn scalar(&self, token: &str) -> Scalar {
    match token {
      "int32" => Scalar::Int32,
      "string" => Scalar::r#String,
      "bool" => Scalar::Bool,
      _ => panic!("Unidentified scalar")
    }
  }

  fn kind(self) -> (Option<&'a str>, Kind<'a>) {
    match self.tokens[0] {
      id @ ("service" | "message") => {
        let name = Some(self.tokens[1]);
        let fields = self
          .children
          .unwrap_or_default()
          .iter()
          .cloned()
          .map(|v| Identifier::identify(v.0, v.1))
          .collect();

        match id {
          "service" => (name, Kind::Service(fields)),
          "message" => (name, Kind::Message(fields)),
          _ => unreachable!()
        }
      }
      "syntax" => (None, Kind::Syntax(self.tokens[3])),
      "package" => (None, Kind::Package(self.tokens[1])),
      _ => (None, Kind::Unknown)
    }
  }

  fn field(self) -> Field<'a> {
    match self.tokens[0] {
      "message" | "service" => {
        let (identifier, kind) = self.kind();

        Field::Block(Block {
          identifier,
          kind
        })
      }
      "rpc" => Field::Rpc(Rpc {
        name: self.tokens[1],
        params: (self.tokens[3], self.tokens[7])
      }),
      val => Field::Property(Property {
        r#type: self.scalar(val),
        name: self.tokens[1],
        value: self.tokens[3]
          .parse()
          .expect("Invalid value for field")
      })
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn identify_kind() {
    let tokens = vec!["message", "Foo", "{", "}"];
    let result = Identifier::identify::<(Option<&str>, Kind)>(tokens, None);

    assert_eq!(result, (Some("Foo"), Kind::Message(Vec::new())));
  }

  #[test]
  fn identify_field() {
    let tokens = vec!["int32", "foo", "=", "1", ";"];
    let result = Identifier::identify::<Field>(tokens, None);

    assert_eq!(
      result,
      Field::Property(Property {
        r#type: Scalar::Int32,
        name: "foo",
        value: 1
      })
    );
  }
}
