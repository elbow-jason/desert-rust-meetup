fn main() {
    println!("Hello, world!");
    {
        let x = box 5i;
    }
}



/*
{
    int *x = malloc(sizeof(int));

    // we can now do stuff with our handle x
    *x = 5;

    free(x);
}
*/
