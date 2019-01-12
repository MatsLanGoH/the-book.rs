// Solution for Exercise 07 - Area of a Rectangular Room
use rectangular_room::Area;
use rectangular_room::get_unit;
use rectangular_room::get_input_as_u32;

fn main() {
    // Set input to feet or meters
    let unit = get_unit();
    
    // Get input dimensions
    print!("What is the length of the room in {}? ", &unit);
    let length = get_input_as_u32();

    print!("What is the width of the room in {}? ", &unit);
    let width = get_input_as_u32();

    // Get area dimensions 
    let area = Area::new(length, width, &unit);

    // Print result output to stdout
    println!("You entered dimensions of {} by {} {}.",
        length,
        width,
        unit,
    );
    println!("The area is
{} square feet
{} square meters",
    area.area_in_feet,
    area.area_in_meters);
}
