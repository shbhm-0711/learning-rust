/*
| Control Flow-Contructs
+ if expressions
+ else if for multiple conditions
+ Loops for repetition of code
* Repeating code with loop
* Conditional loops with while
* Looping through a condition with for
*/
fn main() {
    if_else_loop();
    loops_shit();
}

fn loops_shit() {
    loop {
        println!("Again!");
        break;
        /*
         * Use break in if else condition
         * to break out of loop when needed
         */
    }
    let mut number: f32 = 3.0;

    while number != 0.0 {
        println!("{}!", number);
        number = number - 0.5;
    }

    let arr = [1, 2, 3, 4, 5];

    for element in arr.iter() {
        // * arr.iter()- creates an iterable object
        println!("Value is = {}", element);
    } // * for loop in Rust is safe
      // * and we dont need to deal with id/counter
}

fn if_else_loop() {
    let a = 6;

    if a < 10 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }

    if a % 4 == 0 {
        println!("A is divisible by 4");
    } else if a % 3 == 0 {
        println!("A is divisible by 3");
    } else if a % 2 == 0 {
        println!("A is divisible by 2");
    } else {
        println!("A is not divisible by 2, 3 and 4");
    }
}
