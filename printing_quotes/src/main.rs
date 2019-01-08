// Solution for Exercise 03 - Printing Quotes
use std::io;

fn main() {
    // Prompt for a quote
    println!("What is the quote?");
    let quote = get_input();

    // Prompt for an author
    println!("Who said it?");
    let author = get_input();

    // Create and print quote
    match Quote::new(author, quote) {
        Ok(q) => q.print(),
        Err(e) => println!("Error: {}", e),
    }
}

fn get_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(e) => println!("error: {}", e),
    }

    input.trim().to_string()
}

struct Quote {
    author: String,
    quote: String,
}

impl Quote {
    fn new(author: String, quote: String) -> Result<Quote, &'static str> {
        let author = author;
        let quote = quote;

        if author.chars().count() == 0 {
            return Err("You need to enter an author!");
        } else if quote.chars().count() == 0 {
            return Err("You need to enter a quote!");
        }

        Ok(Quote { author, quote })
    }

    fn print(self) -> () {
        let output = self.author + " says, \"" + &self.quote + "\"";
        println!("{}", output);
    }
}
