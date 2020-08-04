use std::collections::HashMap;

fn main() {
  let extracted =
    extract("please help me, ive been stuck in this basement for several years. he hasnt let me go since he captured me. please send help.");

  println!("{:?}", extracted);
}

fn extract(x: &'static str) -> HashMap<char, u32> {
  let mut chars: HashMap<char, u32> = HashMap::new();

  for c in x.chars() {
    if c == ' ' || c == ',' {
      continue;
    }
    *chars.entry(c).or_insert(0) += 1;
  }

  let mut highest = 0;
  for cap in chars.values() {
    if cap > &highest {
      highest = *cap;
    }
  }

  chars = chars
    .into_iter()
    .filter(|&(_k, v)| v == highest)
    .collect();

  chars
}
