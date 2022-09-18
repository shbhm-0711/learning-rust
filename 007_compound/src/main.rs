/*
|Compound Types
+ Tuples
* Group together a variety of types
* Comma-seperated list of values inside parentheses
* Destructuring breaks a tuple into parts
* Access a tuple element directly by using a period (.)
+ Arrays
* A collection of multiple values
* Every element must have same type
* Arrays in Rust have a fixed length
*/

fn main() {
    // | Tuples
    let tup: (i32, f64, u8) = (500, 4.56, 1);
    let (x, y, z) = tup;
    let a = tup.0;
    let b = tup.1;
    let c = tup.2;

    println!("x{} y{} z{}", x, y, z);
    println!("a{} b{} c{}", a, b, c);

    // | Arrays
    let m_l = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("first {}", m_l[0]);
    println!("Eleventh {}", m_l[10]);
    /*
    fk   if u use out of bound index
    fk   it will compile but panic during runtime
    fk   and block memory access
    */
    /*
    |Safety In Rust
    * Invalid array element accesses
    * Index out of bound-one of the first examples of Rust's safety principles
    * Rust immediately exists instead of allowing the memory access
    + that makes Rust better

    */
}
