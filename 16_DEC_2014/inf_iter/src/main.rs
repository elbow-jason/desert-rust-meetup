
fn main() {
  for mut y in std::iter::count(1i, 5i).map(|x| x+1) {
    y = y + 1;
    println!("y is {}", y);
  }
}

