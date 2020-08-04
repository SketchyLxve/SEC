pub mod char_counter;
pub mod insertion_sort;

fn main() {
  let extracted =
    char_counter::count("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam lacinia diam condimentum euismod eleifend. Proin faucibus urna in sagittis pharetra. Integer ut velit et orci rutrum ullamcorper vitae sit amet orci. Maecenas non nisi in magna feugiat accumsan a sed justo. Praesent imperdiet augue velit, eu dapibus lectus luctus a. Ut fringilla quis mi quis ultrices. Proin rhoncus purus ex, euismod bibendum augue viverra venenatis. Maecenas magna neque, tempus nec massa eu, dictum elementum ex. Aliquam ut sapien consequat, viverra ipsum at, tempus ex. Proin non convallis odio. Curabitur tristique augue quam, at maximus turpis posuere et. Maecenas hendrerit tortor quis maximus efficitur. Duis ante tellus, dignissim id aliquet auctor, bibendum ut nisi. Sed velit nisl, consequat eget aliquet sit amet, condimentum a ligula. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos.");

  insertion_sort::insertion_sort(&mut [3, 5, 7, 8, 9, 1, 2, 6, 4, 22, 34, 562, 4123, 12, 35, 657]);
  println!("{:?}", extracted);
}
