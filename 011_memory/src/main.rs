fn main() {
    let mut s = String::from("Hello");
    /*
     * String::from()-req for memory
     | Other languages.
     * With GC(Garbage collector)- it automatically keeps track of the memory
     * which is no longer being used and cleans it
     * Withou GC- programmer's responsibility to return just like requested.
     * (diff to do correctly)-early:invalid variable,late:wasting of memory,
     * twice-bug.
     * Challenge-pair one allocation with one free(freeing of memory).
     | Rust
     + In Rust-Memory is automatically returned once var goes out of scope.
     * {
     * let s = String::from("hello")//s is valid now, do stuff using s
     * } //this scope is now over, s is no longer valid
     * drop function is called when it goes out of scope.
     *
    */

    s.push_str(", world!");

    println!("{}", s);
    // | error when forgot !-macro
    /*
    fk       error[E0423]: expected function, found macro `println`
    fk        --> main.rs:6:5
    fk       6      println("{}", s);
    fk              ^^^^^^^ not a function
    fk       help: use `!` to invoke the macro
    fk       6      println!("{}", s);
    fk                     +

    | Memory Allocation
    * The memory must be requested from the operating system at runtime.
    * (heap for string)
    * We need to return this memory to the OS when we're done with our string.
    * if {let s1 = String::from("Hello");
    *     Let s2 = s1;} // here stack is created with pointer to same heap.
    * // Stack has
    * 1. pointer to heap where index and value are stored.
    * 2. length-kb used 3.capacity(recieved from OS in bytes)
    * when they go out of scope rust try to clean memory(same heap)
    * for both variables known as DOUBLE FREE ERROR-Memory safety bug
    | Actually what happens in rust
    *when s1 is copied it is moved to s2 and dropped(moved in rust&invalid)
    * -Ownership shit, it will solve problem and only s2 will copy data
    * shallow copy in other languages. To deep copy use clone method.
    */
}
