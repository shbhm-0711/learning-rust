fn main() {
    let x = 5;
    // | creats variable (immutable by default)

    println!("Value of x={}", x);
    // | {}:used for template literals, here for placeholder
    // fk x=6 not possible since it is immutable by default

    let mut y = 5; // * mutable variable
    println!("Value of y={}", y);
    y = 10;
    // + changing is fine

    println!("Value of y-new={}", y);
}
