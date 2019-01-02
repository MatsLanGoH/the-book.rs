use std::io;


fn main() {

    loop {
        // Read temperature to be converted to
        println!("Do you want to convert temperature to (F)ahrenheit or (C)elsius?");

        let mut target_unit = String::new();

        io::stdin().read_line(&mut target_unit)
            .expect("Failed to read line");

        let target_unit: char = match target_unit.trim().parse() {
            Ok(unit) => unit,
            Err(_) => {
                println!("Please input either (F) or (C).");
                break
            },
        };

        // Read temperature
        println!("Please enter a temperature");

        let mut temperature = String::new();

        io::stdin().read_line(&mut temperature)
            .expect("Failed to read temperature");

        let temperature: f32 = match temperature.trim().parse() {
            Ok(temp) => temp,
            Err(_) => {
                println!("Please enter a valid temperature.");
                break
            },
        };

        // Convert temperature

        // Output temperature
        println!("target_unit {}", target_unit);
        println!("temperature {}", temperature);
        break;

    }
}
