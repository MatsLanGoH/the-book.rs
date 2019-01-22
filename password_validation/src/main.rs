// Solution for Exercise 15 - Password validation
//
// If correct password, display "Welcome"
//
// Otherwise print "I don't know you"
use std::collections::HashMap;
use bcrypt::{hash, verify};
use password_validation::get_input;

fn main() {
    // Create a hash map of usernames/passwords
    // Note: Apparently HashMaps aren't supported that well in std
    // https://doc.rust-lang.org/book/ch08-03-hash-maps.html
    let mut accounts = HashMap::new();

    // Insert a few accounts
    accounts.insert(
        String::from("hacker"),
        hash("hunter2", 4).unwrap()
    );
    accounts.insert(
        String::from("captain"),
        hash("12345", 4).unwrap()
    );
    accounts.insert(
        String::from("crustacean"),
        hash("abc$12345", 4).unwrap()
    );

    // Prompt for username and see if it exists
    let username = get_input("What is your username?");

    // Look up user (display a message if not exists)
    if let Some(v) = accounts.get(&username) {
        let password = get_input("What is the password?");
        println!("{:#?}", verify(&password, &v));
        println!("{:#?}", v);
    } else {
        println!("User not found!");
    }

    // If user is correct, prompt for password

    // Ensure that password is correct

}
