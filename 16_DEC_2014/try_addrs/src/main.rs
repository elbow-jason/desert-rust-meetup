fn main() {
    let mut x = box 5i;
    println!("{:p} main: xBox location", &x);
    println!("{:p} main: unboxed xBox location", &*x);
    x = add_one(x);
    println!("{:p} main: xBox location", &x);
    println!("{:p} main: unboxed xBox location", &*x);
    x = add_one(x);
    println!("{:p} main: xBox location", &x);
    println!("{:p} main: unboxed xBox location", &*x);
    x = add_one(x);
    println!("{:p} main: xBox location", &x);
    println!("{:p} main: unboxed xBox location", &*x);
}
fn add_one(mut num: Box<int>) -> Box<int> {
    //dereference and add one to the value of the boxed int
    println!("{:p} add_one: numBox location", &num);
    println!("{:p} add_one: unboxed numBoxlocation", &*num);
    *num += 1;
    //give the Box<int> back...
    num
}