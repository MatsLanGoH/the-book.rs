use std::collections::HashMap;
use std::io;
use std::io::Write;

use bcrypt::hash;

pub fn get_input(prompt: &str) -> String {
    print!("{} ", prompt);
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Something terrible happened here");

    input.trim_end().to_string()
}


pub fn generate_accounts() -> HashMap<String, String> {
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

    accounts
}
