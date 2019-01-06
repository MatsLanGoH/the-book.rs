// Solution for Exercise 01 - Saying Hello
use std::io;

fn main() {
    let input = get_input();

    let output = generate_output(input);

    println!("{}", output);
}

fn get_input() -> String {
    println!("What is your name?");
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

fn generate_output(name: String) -> String {
    format!("{}, nice to meet you!", name)
}

#[test]
fn it_generates_correct_output() {
    let input = "John Doe".to_string();
    assert_eq!(
        "John Doe, nice to meet you!",
        generate_output(input)
    );
}
