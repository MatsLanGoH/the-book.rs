// Solution for Exercise 02 - Counting the Number of Characters
// Prompt for an input string
// Display output that shows
//  - the input string
//  - the number of characters the string contains.
use std::io;
use std::process;


fn main() {
    // ask for input
    println!("What is the input string?");

    // get input
    let input = get_input();

    // use built-in to calculate number of chars
    let word = Word::new(input).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    match word.length {
        // output input string and amount of character(s).
        1 => println!("{w} has {l} character.", w=word.word, l=word.length),
        _ => println!("{w} has {l} characters.", w=word.word, l=word.length),
    }
}

struct Word {
    word: String,
    length: usize,
}

impl Word {
    fn new(word: String) -> Result<Word, &'static str> {
        let word = word;
        let length = word.chars().count();

        // optional: if input is none (empty), tell user they need to input something
        if length == 0 {
            return Err("You need to enter something into the program!");
        }

        Ok(Word { word, length })
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
