// Rust Ownership
// Rust ownership regards resources.
// Memory is a resource.

// Allocating a named variable in memory involves interacting with a
// resource and has 4 concept things involved:

// 1. a name (the name of the variable, this name thing may not apply 
//      in the case of vecs and some other collections).
// 2. a location in memory (this location usually looks like 0x7fff77dbf670).
// 3. a value that is associated with the memory address.
// 4. an ownership "handle" that links the name to the memory.


// Rules for Handles (these rules may be broken in unsafe blocks):
// 1. Only one ownership handle may exists for a resource at any given time
// 2. In order to mutate the value at a memory address an entity(variable)
//      must posess the ownership handle for that resource
// 3. A handle cannot be discarded (memory freed) until all references to a
// resource are finished viewing it.

// ownership involves passing of this handle to a new variable.


/*
{
    int *x = malloc(sizeof(int));

    // we can now do stuff with our handle x
    *x = 5;

    free(x);
}
*/

/*
fn main() {
    println!("Hello, world!");
    {
        let x = box 5i;
    }
}
*/





/*
// runs
fn main() {
    let x = box 5i;

    add_one(x);
}

fn add_one(mut num: Box<int>) {
    *num += 1;
}
*/ 

// won't compile because x is moved
/*
fn main() {
    let x = box 5i;

    add_one(x);

    println!("{}", x);
}

fn add_one(mut num: Box<int>) {
    *num += 1;
}
*/


// ownership
/*
fn main() {
    let x = box 5i;

    let y = add_one(x);

    println!("6 should be... {}", y);
}

fn add_one(mut num: Box<int>) -> Box<int> {
    // dereference and add one to the value of the boxed int
    *num += 1;
    //give the Box<int> back...
    num
}
*/


/*
// x still loses ownership of it's memory address.
// x's handle was moved into add 1 then handed to y.

fn main() {
    let x = box 5i;

    let y = add_one(x);

    println!("6 y should be... {}", y);
    println!("6 x should be... {}", x);
}

fn add_one(mut num: Box<int>) -> Box<int> {
    // dereference and add one to the value of the boxed int
    *num += 1;
    //give the Box<int> back...
    num
}

*/


/*
// ownership not disrupted due to x.clone(),
// but won't change the x because only a copy of the value at x
// was passed to add_one()
fn main() {
    let x = box 5i;

    let y = add_one(x.clone());

    println!("x should be 5 and is actually {}", x);
    println!("y should be 6 and is actually {}", y);
}

fn add_one(mut num: Box<int>) -> Box<int> {
    // dereference and add one to the value of the boxed int
    *num += 1;
    //give the Box<int> back...
    num
}
*/

/*
// ownership of the handle is passes from x into add_one,
// then back from 'num' of add_one() variable to 'x' of main()


fn main() {
    let mut x = box 5i;

    //println!("{:p} main: xBox location", &x);
    println!("{:p} main: unboxed xBox location", &*x);

    x = add_one(x);
    //println!("{:p} main: xBox location", &x);
    println!("{:p} main: unboxed xBox location", &*x);

    x = add_one(x);
    //println!("{:p} main: xBox location", &x);
    println!("{:p} main: unboxed xBox location", &*x);

    x = add_one(x);
    //println!("{:p} main: xBox location", &x);
    println!("{:p} main: unboxed xBox location", &*x);
}


fn add_one(mut num: Box<int>) -> Box<int> {
    // dereference and add one to the value of the boxed int
    //println!("{:p} add_one: numBox location", &num);
    println!("{:p} add_one: unboxed numBoxlocation", &*num);

    *num += 1;
    //give the Box<int> back...
    num
}

*/

// OWNERSHIP ENDS.
// BORROWING FOLLOWS.

/*
fn main() {
    let mut x = box 5i;
    add_one(&mut *x);
    println!("x should be 6 and is actually {}", x);
}

// pass by reference in C++
fn add_one(num: &mut int) {
    *num += 1;
}
*/

// BORROWING IS DONE.
// LIFETIMES FOLLOWS.
// Rust has a feature called 'lifetime elision' which acts like
// a lifetime inference thingy.
// Interestingly, this code:
/*
        fn add_one(num: &int) -> int {
            *num + 1
        }
*/

// After being elided, looks like:
/*
        fn add_one<'a>(num: &'a int) -> int {
            *num + 1
        }
*/

//Read:
//      Define a function 'add_one' with has is assoctiated 
//      with a lifetime of 'a.

//      The function that 'add_one' takes an parameter that is a reference
//      that is associated with a lifetime of 'a.
//      The argument of that parameter is 'num'.

//      The function 'add_one' shall return an integer
//      The function add_one returns the value of dereferenced 'num' plus one.
