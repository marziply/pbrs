use fancy_regex::{Error as RegexError, Regex};

pub fn strip_comments(raw_str: &str) -> Result<String, RegexError> {
  let re = Regex::new(r"//.*")?;
  let result = re.replace_all(raw_str, "");

  Ok(result.to_string())
}

pub fn extract_tokens(raw_str: &str) -> Result<Vec<&str>, RegexError> {
  let re = Regex::new("[[:alnum:]]+|[[:punct:]]")?;
  let result = re
    .captures_iter(raw_str)
    .flat_map(|v| -> Vec<&str> {
      v.unwrap()
        .iter()
        .map(|i| i.unwrap().as_str())
        .collect()
    })
    .collect();

  Ok(result)
}
