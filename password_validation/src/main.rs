// Solution for Exercise 15 - Password validation
//
// Prompt for username
// Prompt for Password
//
// Compare username / password given by the user
//
// If correct password, display "Welcome"
//
// Otherwise print "I don't know you"
use std::collections::HashMap;
use bcrypt::{MIN_COST, hash, verify};

fn main() {
    // Create a hash map of usernames/passwords
    let mut accounts = HashMap::new();

    // Insert a few accounts
    accounts.insert(
        String::from("hacker"),
        hash("hunter2", MIN_COST).unwrap()
    );
    accounts.insert(
        String::from("captain"),
        hash("12345", MIN_COST).unwrap()
    );

    println!("{:#?}", accounts);

    println!("Hello, world!");
}
