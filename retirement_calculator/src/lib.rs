use std::io;
use std::io::Write;

fn get_input() -> String {
    // Flush buffer so we can have input prompt on the same line
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong");

    input.trim().to_string()
}

pub fn get_input_as_i32() -> i32 {
    // Read input and convert to i32
    // Loop until we get a valid input
    loop {
        let input = get_input();

        match input.parse::<i32>() {
            Err(e) => {
                eprintln!("Error: {}.", e);
                print!("Please enter a valid number: ");
                continue;
            }
            Ok(x) => return x,
        }
    }
}

pub fn calculate_years_until_retirement(
    current_age: i32,
    retirement_age: i32
) -> i32 {
    retirement_age - current_age
}
