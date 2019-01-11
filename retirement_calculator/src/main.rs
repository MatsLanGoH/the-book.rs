// Solution for Exercise 06 - Retirement Calculator
//
use chrono::prelude::*;

use retirement_calculator::{calculate_years_until_retirement, get_input_as_i32};

fn main() {
    // Ask for input
    print!("What is your current age? ");
    let current_age = get_input_as_i32();

    print!("At what age would you like to retire? ");
    let retirement_age = get_input_as_i32();

    let years_remaining = calculate_years_until_retirement(current_age, retirement_age);

    if years_remaining > 0 {
        // Get current year
        let current_year = Local::now().year();

        // Output
        println!("You have {} year(s) left until you can retire.", years_remaining);

        println!("It's {}, so you can retire in {retirement_year}.",
            current_year,
            retirement_year = current_year + years_remaining
            );
    } else {
        println!("Congratulations! You can already retire!");
    }
}
