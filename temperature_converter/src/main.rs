use std::io;
extern crate measurements;

use measurements::Temperature;


fn main() {

    loop {
        // Read temperature to be converted to
        println!("Do you want to convert temperature to (F)ahrenheit or (C)elsius?");

        let mut target_unit = String::new();

        io::stdin().read_line(&mut target_unit)
            .expect("Failed to read line");

        let target_unit: char = match target_unit.trim().to_uppercase().parse() {
            Ok(unit) => unit,
            Err(_) => {
                println!("Please input either (F) or (C).");
                break
            },
        };

        let source_unit = if target_unit == 'F' {'C'} else {'F'};

        // Read temperature
        println!("Please enter a temperature");

        let mut temperature = String::new();

        io::stdin().read_line(&mut temperature)
            .expect("Failed to read temperature");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(temp) => temp,
            Err(_) => {
                println!("Please enter a valid temperature.");
                break
            }
        };

        // Convert temperature
        let result = if target_unit == 'F' {
            let temperature = Temperature::from_celsius(temperature);
            temperature.as_fahrenheit()
        } else {
            let temperature = Temperature::from_fahrenheit(temperature);
            temperature.as_celsius()
        };

        // Output temperature
        println!("{}°{} equals {}°{}.", temperature, source_unit, result, target_unit);
        break;
    }
}
