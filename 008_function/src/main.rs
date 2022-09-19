fn main() {
    println!("Hello, world!");
    another_function();
    another_function2(23);
    let a = sum(10, 56);
    println!("Sum = {}", a);
    let b = sum1(1, 4);
    println!("Sum = {}", b)
}

fn another_function() {
    println!("Another fn");
}

fn another_function2(x: i32) {
    println!("value passed x = {}", x);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
    //fk if I add ; in above line it will throw error:
    //fk we defined i32 return type but found ()-empty tuple
    //+ Reasoning - ; in end makes a line into statement.
}

//|Explicit return of value
fn sum1(x: i32, y: i32) -> i32 {
    let a = x + y;
    // a = a * 8;
    return a;
    // *  using return is a statement
}
