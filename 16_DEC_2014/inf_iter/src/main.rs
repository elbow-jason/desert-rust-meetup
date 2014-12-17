
fn main() {
  let mut counted = 0i;
  for mut y in std::iter::count(1i, 1i).map(|x| x+1 ) {
    println!("count is {}", y);
  }
}

