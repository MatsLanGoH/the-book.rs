use std::io;
use std::io::Write;

fn get_input(prompt: &str) -> String {
    // Flush buffer to show prompt on same line as input
    print!("{} ", prompt);
    let _ = io::stdout().flush();
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong");
    
    input.trim().to_string()
}

pub fn get_input_as_u8(prompt: &str) -> u8 {
    // Read input and convert to u32
    // Loop until we get a valid input
    loop {
        let input = get_input(prompt);

        match input.parse::<u8>() {
            Err(e) => {
                eprintln!("Error: {}", e);
                println!("Please enter a valid number!");
                continue;
            },
            Ok(n) if n == 0 => {
                println!("Please enter a number greater than 0!");
                continue;
            }
            Ok(n) => return n,
        }
    }
}