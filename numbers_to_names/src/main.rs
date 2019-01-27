// Solution for Ex21 Numbers to Names
// Input a number
// Output the corresponding month
use numbers_to_names::{
    get_month_from_number,
    get_number_input,
};

fn main() {
    let number = get_number_input();

    let month = get_month_from_number(&number);
    println!("The name of the month is {}.", month);
}
