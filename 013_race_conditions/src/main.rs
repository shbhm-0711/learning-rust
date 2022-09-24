fn main() {
    let refers_nothing = dangle();
    println! {"{}",refers_nothing}
}

fn dangle() -> String {
    let s = String::from("hello");

    // &s;
    // * data race created because main fn trying to read refrence
    // * but dangle dropped it so refrence is invalid
    s
}

/*
| Rules of refrence
* 1. At any given time, you can have either but not both:
* a) One mutable refrence
* b) Any no. of immutable refrences.
* 2. Refrences must always be valid.
*/
