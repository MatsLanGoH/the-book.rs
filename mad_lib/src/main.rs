// Solution for Exercise 04 - Mad Lib
// Prompt for 4 kinds of input
// Display output that shows the result
use std::io;
use std::io::Write;


fn main() {
    print!("Enter a noun: ");
    let noun = get_input();

    print!("Enter a verb: ");
    let verb = get_input();

    print!("Enter an adjective: ");
    let adjective = get_input();

    print!("Enter an adverb: ");
    let adverb = get_input();

    println!(
        "Do you {verb} your {adjective} {noun} {adverb}? That's hilarious!",
        verb=verb,
        noun=noun,
        adverb=adverb,
        adjective=adjective,
    );
}

fn get_input() -> String {
    // Include this to make sure we get input on same line as print!()
    let _ = io::stdout().flush();
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Something went wrong");

    input.trim().to_string()
}
