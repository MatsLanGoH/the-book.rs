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
        if index == 1 {
            suffix = "st";
        } else {
            suffix = "th";
        }
        println!("On the {}{} of Christmas,", index + 1, suffix);
        println!("my true love sent to me:");
        
        println!("{} {}", index + 1, items[index]);
        // Ascii art taken from https://www.asciiart.eu/holiday-and-events/christmas/other
        println!(" .-.  .-.  .-.  .-.  .-.  .-.  .-.  .-.  .-.  .-.  .-.  .-.  .-.");
        println!("(   )(   )(   )(   )(   )(   )(   )(   )(   )(   )(   )(   )(   )");
        println!(" `-'  `-'  `-'  `-'  `-'  `-'  `-'  `-'  `-'  `-'  `-'  `-'  `-'");
    }

}
