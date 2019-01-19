use pizza_party::get_input_as_u8;

// Solution for Exercise 08 - Pizza Party
fn main() {
    // Get input (number of people)
    let number_of_people = get_input_as_u8("How many people?");
    
    // Get input (number of pizzas)
    let number_of_pizzas = get_input_as_u8("How many pizzas do you have?");

    // Get input (number of slices per pizza);
    let slices_per_pizza = get_input_as_u8("How many slices per pizza?");

    // Calculate slices per person and remainder
    let slices_per_person = number_of_pizzas * slices_per_pizza / number_of_people;
    let leftover_slices = number_of_pizzas * slices_per_pizza % number_of_people;
    
    // Output slice(s) per person println!( "{} people with {} pizzas",
    println!("{} people with {} pizza{}.",
        number_of_people,
        number_of_pizzas,
        if number_of_pizzas > 1 {"s"} else {""},
    );
    println!(
        "Each person gets {} slice{} of pizza.",
        slices_per_person,
        if slices_per_person > 1 {"s"} else {""},
    );

    // Output remainder if any
    println!(
        "There {} {} leftover slice{}.",
        if leftover_slices == 1 {"is"} else {"are"},
        leftover_slices,
        if leftover_slices > 1 {"s"} else {""},
    );
}
