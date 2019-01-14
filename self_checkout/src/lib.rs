use std::io;
use std::io::Write;


fn get_input() -> String {
    // Flush buffer so we can have prompt and input on same line
    let _ = io::stdout().flush();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong");

    input.trim_end().to_owned()
}

pub fn get_input_as_u32() -> u32 {
    // Read input and convert to u32
    // Loop until we get a valid input
    loop {
        let input = get_input();

        match input.parse::<u32>() {
            Err(e) => {
                eprintln!("Error: {}", e);
                print!("Please enter a positive number: ");
                continue;
            },
            Ok(x) => return x,
        }
    }
}


pub struct Transaction {
    pub price: u32,
    pub quantity: u32,
    pub subtotal: f32,
}

impl Transaction {
    pub fn new(price: u32, quantity: u32) -> Transaction {
        let price = price;
        let quantity = quantity;
        let subtotal = price as f32 * quantity as f32;

        Transaction { price, quantity, subtotal }
    }
}


pub fn get_tax(amount: f32, rate: f32) -> f32 {
    rate / 100.0 * amount
}
