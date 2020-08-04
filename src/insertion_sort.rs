pub fn insertion_sort(ints: &mut [i64]) -> &mut [i64] {
  println!("Initial = {:?}", ints);
  for i in 0..ints.len() {
    let mut j = i;
    while j > 0 && ints[j - 1] > ints[j] {
      ints.swap(j - 1, j);
      j -=  1;
    }
  }

  println!("Result = {:?}", ints);
  return ints;
}