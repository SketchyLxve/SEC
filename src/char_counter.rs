use std::collections::HashMap;

pub fn count(x: &'static str) -> HashMap<char, u32> {
  let mut chars: HashMap<char, u32> = HashMap::new();

  for c in x.chars() {
    if c == ' ' || c == ',' {
      continue;
    }
    *chars.entry(c).or_insert(0) += 1;
  }

  let highest = *chars.values().max().unwrap_or(&0);

  chars
    .into_iter()
    .filter(|&(_k, v)| v == highest)
    .collect()
}