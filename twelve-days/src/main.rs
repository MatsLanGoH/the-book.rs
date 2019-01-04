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
        "Nine Ladies Dancing",
        "Lords a Leaping",
        "Pipers Piping",
        "Drummers Drumming",
    ];
    
    let mut suffix;

    for index in 0..items.len() {
        let day = index + 1;
        if day == 1 {
            suffix = "st";
        } else if day == 2 {
            suffix = "nd";
        } else if day == 3 {
            suffix = "rd";
        } else {
            suffix = "th";
        }
        println!("On the {}{} day of Christmas,", day, suffix);
        println!("my true love sent to me:");
        
        for gift in (0..=index).rev() {

            if gift == 0 {
                println!("a {}", items[gift]);
            } else {
                println!("{} {}", gift + 1, items[gift]);
                if gift == 1 {
                    print!("and ");
                }
            }
        }

        // Insert after each stanza
        // Ascii art taken from https://www.asciiart.eu/holiday-and-events/christmas/other
        println!(" .-.  .-.  .-.  .-.  .-.  .-.  .-.  .-.  .-.  .-.  .-.  .-.  .-.");
        println!("(   )(   )(   )(   )(   )(   )(   )(   )(   )(   )(   )(   )(   )");
        println!(" `-'  `-'  `-'  `-'  `-'  `-'  `-'  `-'  `-'  `-'  `-'  `-'  `-'");
        // println!("{}", parse_integer_to_number(1));
    }
}

