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
        println!("On the {}{} of Christmas,", index, suffix);
        println!("{}", items[index]);
    }

}
