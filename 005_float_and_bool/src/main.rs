/*
+ f32
+ f64

+ bool
*/

fn main() {
    let x = 2.0; // *by default 64 bit float
    let x1: f64 = 4.0; // *by default 64 bit float

    println!("default f64 = {}", x);
    println!("manuaml f64 = {}", x1);

    let y: f32 = 3.0; // * manually assigned 32 bit float

    println!("manuaml f32 = {}", y);
    let t = true; // * default bool
    let f: bool = false; // * explicit bool
    println!("{}", t);
    println!("{}", f);
}
