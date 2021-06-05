use std::io;

fn main() {
    println!("Input a number");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: i32 = input
        .trim()
        .parse()
        .expect("Please input a positive number");
    if n < 0 {
        panic!("Please input a positive number");
    }

    let result = fibonacci(n);
    println!("n is {}, result is {}", n, result);
}

fn fibonacci(n: i32) -> i32 {
    if n == 0 {
        1
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
