/*
|Ownership - Stack for simple/primitive(not exactly) data types
|Heap for complex
+ Each value in Rust has a variable that's called its owner.
+ There can only be one owner at a time.
+ When the owner goes out of scope, the value gets dropped.
*/

fn main() {
    let x = 1; // * {}-defines scope
    if x == 1 {
        let a = 10;
        println!("A = {}", a);
    };
    // | Can't access a variable here.
}
