/*
| types of Variables(Data Types)
+Scalar Type
*integers
*Floating Point Numbers
*Boolean
*Characters
+Compound Type
*/

fn main() {
    let mut x = 2147483647; // *by default int 32 bit and manually made to mutable
    println!("{}", x);
    x = x - 1; // * will work as less than 32 bit int max limit
    println!("{}", x);
    let mut x1: i32 = 2147483647; // * manually assigned int 32 bit and mutable
    println!("{}", x1);
    x1 = x1 - 1;
    println!("{}", x1);
    //+ same thing as x
    /*
    fk * x=x+1;
    fk * above line will overflow default 32 bit int rust sets
    */
    let mut y: i64 = 2147483647; // * type set to 64 bit int

    println!("{}", y);

    y = y + 1; // * Will compile

    println!("{}", y);
}
