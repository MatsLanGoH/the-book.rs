use std::io;
use std::io::Write;

pub fn get_input(prompt: &str) -> String {
    print!("{} ", prompt);
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Something terrible happened here");

    input.trim_end().to_string()
}

