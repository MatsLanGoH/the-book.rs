// Solution for Exercise 10 - Self-Checkout
use self_checkout::{get_input_as_u32, get_tax};
use self_checkout::Transaction;

fn main() {
    // Get input and store it
    let mut transactions = Vec::new();

    for i in 1..4 {
        print!("Enter the price of item {}: ", i);
        let item_price = get_input_as_u32();
        print!("Enter the quantity of item {}: ", i);
        let item_qty = get_input_as_u32();

        let t = Transaction::new(item_price, item_qty);
        transactions.push(t);
    }

    // Calculate subtotal
    let mut subtotal: f32 = 0.0;

    for item in transactions.iter() {
        subtotal += item.subtotal;
    }

    println!("Subtotal: ${:.2}", subtotal);

    // Calculate tax
    let tax = get_tax(subtotal, 5.5);
    println!("Tax: ${:.2}", tax);

    // Output total
    println!("Total: ${:.2}", subtotal + tax);
}
