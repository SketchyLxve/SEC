pub mod char_counter;

fn main() {
  let extracted =
    char_counter::count("please help me, ive been stuck in this basement for several years. he hasnt let me go since he captured me. please send help.");

  println!("{:?}", extracted);
}