use regex::{Error as RegexError, Regex};

type TokenResult<T> = Result<T, RegexError>;

type TokenVector<'a, T> = TokenResult<Vec<T>>;

fn strip_comments(raw_str: &str) -> TokenResult<String> {
  let re = Regex::new(r"//.*")?;
  let result = re.replace_all(raw_str, "");

  Ok(result.to_string())
}

fn into_tokens<'a>(raw_str: &str) -> TokenVector<String> {
  let re = Regex::new("[[:alnum:]]+|[[:punct:]]")?;
  let result = re
    .captures_iter(raw_str)
    .flat_map(|v| {
      v.iter()
        .map(|i| i.unwrap().as_str().to_string())
        .collect::<Vec<String>>()
    })
    .collect();

  Ok(result)
}

pub fn translate<'a>(input: &str) -> TokenVector<String> {
  let stripped = strip_comments(input)?;
  let tokens = into_tokens(&stripped)?;

  Ok(tokens)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn comments_removed() -> Result<(), RegexError> {
    let input = "
      // comment
      message Foo {
        string bar = 1;
      }
    ";
    let result = strip_comments(input)?;

    assert!(!result.contains("// comment"));

    Ok(())
  }

  #[test]
  fn translate_service() -> Result<(), RegexError> {
    let input = "
      service Foo {
        rpc Bar (Request) returns (Response) {}
      }
    ";
    let result = translate(input)?;
    let expect = vec![
      "service", "Foo", "{", "rpc", "Bar", "(", "Request", ")", "returns", "(",
      "Response", ")", "{", "}", "}",
    ];

    assert_eq!(result, expect);

    Ok(())
  }

  #[test]
  fn translate_message() -> Result<(), RegexError> {
    let input = "
      message Foo {
        int32 a = 1;
        string b = 2;
        bool c = 3;
      }
    ";
    let result = translate(input)?;
    let expect = vec![
      "message", "Foo", "{", "int32", "a", "=", "1", ";", "string", "b", "=",
      "2", ";", "bool", "c", "=", "3", ";", "}",
    ];

    assert_eq!(result, expect);

    Ok(())
  }
}
