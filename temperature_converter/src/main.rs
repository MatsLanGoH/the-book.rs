use std::io;


fn main() {
    // Read temperature to be converted to
    println!("Do you want to convert temperature to (F)ahrenheit or (C)elsius?");

    let mut target_unit = String::new();

    io::stdin().read_line(&mut target_unit)
        .expect("Failed to read line");

    loop {
        let target_unit: char = match target_unit.trim().parse() {
            Ok(unit) => unit,
            Err(_) => {
                println!("Please input either (F) or (C).");
                break
            },
        };

        // Read temperature
        // Convert temperature
        // Output temperature
        println!("target_unit {}", target_unit);
        break;

    }



}
