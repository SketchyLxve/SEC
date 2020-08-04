use std::collections::HashMap;
use std::time::Instant;

pub fn count(x: &'static str) -> HashMap<char, u32> {
  let now = Instant::now();
  let mut chars: HashMap<char, u32> = HashMap::new();
  let captures = x
    .chars()
    .filter(|&x| x != ' ' && x != ',')
    .collect::<Vec<char>>();

  for c in captures {
    *chars.entry(c).or_insert(0) += 1;
  }

  let highest = *chars.values().max().unwrap_or(&0);
  let end = now.elapsed();
  println!("{:?}", end);
  chars.into_iter().filter(|&(_k, v)| v == highest).collect()
}