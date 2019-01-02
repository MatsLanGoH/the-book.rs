use std::io;


fn main() {
    println!("Hello, world!");

    let mut n = String::new();

    io::stdin().read_line(&mut n)
        .expect("Failed to read line");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Please enter a positive integer."),
    };

    let result = fibonacci(n);

    println!("{}", result);
}


fn fibonacci(num: u32) -> u32 {
    if num == 0 || num == 1 {
        1
    } else {
        fibonacci(num-1) + fibonacci(num-2)
    }
}
