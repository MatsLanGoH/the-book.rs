fn main() {
    let items = [
        "Partridge in a Pear Tree",
        "Turtle Doves",
        "French Hens",
        "Calling Birds",
        "Golden Rings",
        "Geese a Laying",
        "Swans a Swimming",
        "Maids a Milking",
        "Ladies Dancing",
        "Lords a Leaping",
        "Pipers Piping",
        "Drummers Drumming",
    ];
    
    for index in 0..items.len() {
        let mut day = index + 1;
        println!("On the {} day of Christmas,", parse_integer_to_nth_day(&day));
        println!("my true love sent to me:");
        
        for gift in (0..=index).rev() {

            println!("{} {}", parse_integer_to_word(&day), items[gift]);
            if gift == 1 {
                print!("and ");
            }

            day -= 1;
        }

        // Insert after each stanza
        // Ascii art taken from https://www.asciiart.eu/holiday-and-events/christmas/other
        println!(" .-.  .-.  .-.  .-.  .-.  .-.  .-.  .-.  .-.  .-.  .-.  .-.  .-.");
        println!("(   )(   )(   )(   )(   )(   )(   )(   )(   )(   )(   )(   )(   )");
        println!(" `-'  `-'  `-'  `-'  `-'  `-'  `-'  `-'  `-'  `-'  `-'  `-'  `-'");
    }
}

fn parse_integer_to_word(integer: &usize) -> std::string::String {
    // TODO: Find a more elegant way to handle titlecase
    // Ideally, we would have lowercase strings and capitalize the first letter as needed
    // TODO: Return a Stringified number if we have a number larger than 12
    let word = match *integer {
        1 => "a",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        11 => "Eleven",
        12 => "Twelve",
        _ => "", 
    };
    word.to_string()
}

fn parse_integer_to_nth_day(integer: &usize) -> std::string::String {
    let nth_day = match *integer {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "", 
    };
    nth_day.to_string()
}