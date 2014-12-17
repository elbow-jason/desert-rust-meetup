use std::iter::repeat;

fn main() {
  // THIS WORKS LIKE MAGIC.
  for z in std::iter::count(1i, 5i).map(|x| println!("x is {}", x)) {};

    //let x = vec![1];
    //x.iter(|y| { println!("y is {}", y); } ) ;

}

