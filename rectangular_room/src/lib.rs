use std::io;
use std::io::Write;

const SQUARE_FT_TO_METERS: f32 = 0.09290304;

fn get_input() -> String {
    // Flush buffer so we can have input on same line as prompt
    let _ = io::stdout().flush();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong");
    
    input.trim().to_string()
}

pub fn get_input_as_u32() -> u32 {
    // Read input and convert to u32
    // Loop until we get a valid input
    loop {
        let input = get_input();

        match input.parse::<u32>() {
            Err(e) => {
                eprintln!("Error: {}", e);
                print!("Please enter a valid integer: ");
                continue;
            },
            Ok(x) => return x,
        }
    }
}

pub struct Area {
    pub area_in_feet: f32,
    pub area_in_meters: f32,
}

impl Area {
    pub fn new(width: u32, length: u32, unit_type: &String) -> Area {
        let width: f32 = width as f32;
        let length: f32 = length as f32; 
        
        if unit_type == "feet" {
                let area_in_feet = width * length;
                let area_in_meters = &area_in_feet * SQUARE_FT_TO_METERS;
                Area { area_in_feet, area_in_meters }
            } else {
                let area_in_meters = width * length;
                let area_in_feet = area_in_meters / SQUARE_FT_TO_METERS;
                Area { area_in_feet, area_in_meters }
            }
    }
}


pub fn get_unit() -> String {
    print!("Would you like to enter the room dimensions in (f)eet or (m)eters? ");
    loop {
        let input = get_input();
        match input.as_ref() {
            "f" => return "feet".to_string(),
            "m" => return "meters".to_string(),
            _ => {
                print!("Please input either \"f\" or \"m\". ");
                continue;
            }
        };
    }
}