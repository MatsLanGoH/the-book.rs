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
use bcrypt::{hash, verify};

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

    // Look up user (display a message if not exists)
    let input = String::from("hacker");
    let lookup = accounts.get(&input);
    match lookup {
        Some(v) => {
            println!("{:#?}", v);
        }
        None => {
            println!("User not found");
        },
    };

    

    println!("Hello, world!");
}
