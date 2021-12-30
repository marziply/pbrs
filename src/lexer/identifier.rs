use super::{Block, Field, Kind, Property, Rpc, Scalar, TokenChildren};

pub struct Identifier<'a> {
  pub tokens: Vec<&'a str>,
  pub children: TokenChildren<'a>
}

impl<'a> From<Identifier<'a>> for (Option<String>, Kind) {
  fn from(value: Identifier<'a>) -> Self {
    value.kind()
  }
}

impl<'a> From<Identifier<'a>> for Field {
  fn from(value: Identifier<'a>) -> Field {
    value.field()
  }
}

impl<'a> Identifier<'a> {
  pub fn create<T>(tokens: Vec<&'a str>, children: TokenChildren<'a>) -> T
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

  fn field(self) -> Field {
    match self.tokens[0] {
      "message" | "service" => {
        let (identifier, kind) = self.kind();

        Field::Block(Block {
          identifier,
          kind
        })
      }
      "rpc" => Field::Rpc(Rpc {
        name: self.token(1),
        params: (self.token(3), self.token(7))
      }),
      val => Field::Property(Property {
        r#type: self.scalar(val),
        name: self.token(1),
        value: self.tokens[3]
          .parse()
          .expect("Invalid value for field")
      })
    }
  }

  fn kind(self) -> (Option<String>, Kind) {
    match self.tokens[0] {
      id @ ("service" | "message") => {
        let name = Some(self.tokens[1].to_string());
        let fields = self
          .children
          .unwrap_or_default()
          .iter()
          .cloned()
          .map(|v| Identifier::create(v.0, v.1))
          .collect();

        match id {
          "service" => (name, Kind::Service(fields)),
          "message" => (name, Kind::Message(fields)),
          _ => unreachable!()
        }
      }
      "syntax" => (None, Kind::Syntax(self.tokens[3].to_string())),
      "package" => (None, Kind::Package(self.tokens[1].to_string())),
      _ => (None, Kind::Unknown)
    }
  }

  fn token(&self, index: usize) -> String {
    self.tokens[index].to_string()
  }
}
