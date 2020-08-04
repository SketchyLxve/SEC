use std::time::Instant;

pub fn insertion_sort(ints: &mut [i64]) -> &mut [i64] {
  let now = Instant::now();
  println!("Initial = {:?}", ints);
  for mut i in 0..ints.len() {
    while i > 0 && ints[i - 1] > ints[i] {
      ints.swap(i - 1, i);
      i -=  1;
    }
  }

  println!("Result = {:?}", ints);
  println!("End! {:?}", now.elapsed());
  ints
}