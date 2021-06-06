fn main() {
    let s = String::from("Hello"); // heap
    takes_ownership(s); // move

    let x = 5; // stack
    makes_copy(x); // do nothing
}

fn takes_ownership(some_string: String) {
    println!("string is {}", some_string); // drop
}

fn makes_copy(some_integer: i32) {
    println!("integer is {}", some_integer); // do nothing
}
