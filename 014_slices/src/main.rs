fn main() {
    let s = String::from("Hello, World!");

    let hello = &s[0..5];

    println!("hello contains = {}", hello)
}

/*
|String Literals are slices
* let s = "Hellow";
* above, &str(type) is an immutabel refrence.

| Other Slices
+ let a = [1,2,3,4,5];
+ let slice = &a[1..3];
* type is &[i32]
* works same way as string literal.

|
*/
