use fancy_regex::{Error as RegexError, Regex};

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
      v.unwrap()
        .iter()
        .map(|i| i.unwrap().as_str().to_string())
        .collect::<Vec<String>>()
    })
    .collect();

  Ok(result)
}

pub fn translate<'a>(input: String) -> TokenVector<'a, String> {
  let stripped = strip_comments(input.as_str())?;
  let tokens = into_tokens(stripped.as_str())?;

  Ok(tokens)
}
