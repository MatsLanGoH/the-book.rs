// Solution for Exercise 15 - Password validation

use bcrypt::verify;
use password_validation::{
    generate_accounts,
    get_input,
    get_password,
};

fn main() {
    // Get accounts
    let accounts = generate_accounts();

    // Prompt for username and see if it exists
    let username = get_input("What is your username?");
    if let Some(hash) = accounts.get(&username) {

        // If user is correct, prompt for password
        let password = get_password("What is the password?");

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
