// Solution for Exercise 15 - Password validation

use std::collections::HashMap;
use bcrypt::verify;
use password_validation::{
    generate_accounts,
    get_input,
};

fn main() {
    // Get accounts
    let accounts = generate_accounts();

    // Prompt for username and see if it exists
    let username = get_input("What is your username?");
    if let Some(hash) = accounts.get(&username) {

        // If user is correct, prompt for password
        let password = get_input("What is the password?");
        // Ensure that password is correct
        if let Ok(true) = verify(&password, &hash) {
            println!("Welcome!");
        } else {
            println!("I don't know you.");
        }

    } else {
        println!("User not found!");
    }


}
