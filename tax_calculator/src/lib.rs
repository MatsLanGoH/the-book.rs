use std::io;
use std::io::Write;

// What if we've got multiple states? Use a hash map?
const TAX_RATE_WI: f32 = 5.5;

pub fn get_input_as_string(prompt: &str) -> String {
    // FLush buffer so we can have input on same line as prompt
    print!("{} ", prompt);
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong");

    input.trim().to_string()
}

pub fn get_input_as_f32(prompt: &str) -> f32 {
    // Read input and convert to f32
    // Loop until we get a valid input
    loop {
        let input = get_input_as_string(prompt);

        match input.parse::<f32>() {
            Err(e) => {
                eprintln!("Error: {}", e);
                println!("Please enter a valid number!");
                continue;
            },
            // We don't want negative values either
            Ok(n) if n < 0.0 => {
                println!("Please enter a positive number!");
                continue;
            },
            Ok(n) => return n,
        }
    }
}

// Define US States as an enum so we can easily check tax rates later.
enum UsState{
    WI,
    Other,
}

fn set_us_state (state: String) -> UsState {
    match state.to_ascii_lowercase().as_ref() {
        // Note: Is there a better way to look this up?
        "wi" => UsState::WI,
        "wisconsin" => UsState::WI,
        _ => UsState::Other, 
    }
}

// This will hold the order given by a user
pub struct Order {
    pub amount: f32,
    resident_state: UsState,
}

impl Order {
    pub fn new(amount: f32, resident_state: String) -> Order {
        let resident_state = set_us_state(resident_state);

        Order {
            amount,
            resident_state,
        }
    }
}


// And this will calculate the check
// Optional TODO: Maybe add an `add` function
// that allows us to add more orders to the check.
pub struct Check {
    pub subtotal: f32,
    pub tax: f32,
    pub total: f32,
}

impl Check {
    pub fn new(order: &Order) -> Check {
        let subtotal = order.amount;

        let tax = match order.resident_state {
            UsState::WI => TAX_RATE_WI,
            UsState::Other => 0.0,
        };

        let total = order.amount;

        Check {
            subtotal,
            tax,
            total,
        }
    }

    pub fn output_total(&mut self) -> String {
        let mut output = String::new();

        if self.tax != 0.0 {
            let tax_amount = get_tax_amount(&self.subtotal, &self.tax);

            output.push_str(&format!("The subtotal is ${:.2}.\n", self.subtotal));
            output.push_str(&format!("The tax is ${:.2}.\n", tax_amount));
            self.total += tax_amount;
        }
        output.push_str(&format!("The total is ${:.2}.", self.total));

        output
    }
}

fn get_tax_amount(subtotal: &f32, tax: &f32) -> f32 {
    (subtotal * tax).round() / 100.0
}
