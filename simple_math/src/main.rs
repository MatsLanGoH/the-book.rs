// Solution for Exercise 05 - Simple Mah
use std::io;
use std::io::Write;

fn main() {
    // Get input as strings
    print!("What is the first number? ");
    let first: i64 = get_positive_integer_input();

    print!("What is the second number? ");
    let second: i64 = get_positive_integer_input();

    // do math
    let sum = &first + &second;
    let difference = &first - &second;
    let product = &first * &second;
    let quotient = first / second;

    // Output statement
    println!(
        "{first} + {second} = {sum}
{first} - {second} = {difference}
{first} * {second} = {product}
{first} / {second} = {quotient}",
    first=first,
    second=second,
    sum=sum,
    difference=difference,
    product=product,
    quotient=quotient
    );
}

fn get_input() -> String {
    // flush to stdout so we can have input on same line as input query
    let _ = io::stdout().flush();
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Something went wrong");
    input.trim().to_string()
}

fn get_positive_integer_input() -> i64 {
    loop {
        let input = get_input();

        match input.parse::<i64>() {
            Err(err) => {
                eprint!("Error: {}", err);
                continue;
            },
            Ok(x) => {
                if x > 0 {
                    return x;
                } else {
                    eprint!("Please enter a positive integer. ");
                    continue;
                }
            },
        }
    }
}
