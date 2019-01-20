// Solution for Ex14 - Tax Calculator

use tax_calculator::{
    get_input_as_f32,
    get_input_as_string,
    Order,
    Check,
};

fn main() {
    // Prompt for order amount
    let amount = get_input_as_f32("What is the order amount?");

    // Prompt for state
    let resident_state = get_input_as_string("What is the state?");

    // Create order from our data
    let order = Order::new(amount, resident_state); 

    // Create a check (needs to be mutable so we can calculate the result)
    let mut check = Check::new(&order);

    // Retrieve and print total
    println!("{}", check.output_total());
}
