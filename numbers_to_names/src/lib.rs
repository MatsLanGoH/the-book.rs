use std::io;
use std::io::Write;

fn get_input(prompt: &str) -> String {
    print!("{} ", prompt);
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong.");

    input.trim_end().to_owned()
}


pub fn get_number_input() -> u8 {
    // Loop until we have a valid input
    loop {
        let input = get_input("Please enter the number of the month.");

        match input.parse::<u8>() {
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            },
            Ok(n) if (n < 1 || n > 12) => {
                println!("Please enter a number between 1 and 12.");
                continue;
            },
            Ok(n) => return n,
        };
    }
}


pub fn get_month_from_number(number: &u8) -> &str {
    match number {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "unknown",
    }
}
