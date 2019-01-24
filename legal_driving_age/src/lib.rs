use std::io;
use std::io::Write;

fn get_input(prompt: &str) -> String{
    print!("{} ", prompt);
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong");

    input.trim().to_string()
}

pub fn get_age() -> u8 {
    // Loop to repeat prompt until a valid age is given
    loop {
        let age_string = get_input("What is your age?");
        match age_string.parse::<u8>() {
            Ok(0) => {
                println!("Please enter a valid age.");
                continue;
            },
            Ok(num) => return num,
            Err(_) => {
                println!("Please enter a valid age.");
                continue;
            },
        };
    }
}