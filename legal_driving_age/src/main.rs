// Solution for Exercise 16 - Legal Driving Age
use legal_driving_age::get_age;

fn main() {
    let mut is_legal_age = false;

    let age = get_age();
    if let true = age >= 16 {
        is_legal_age = true;
    } else {
        is_legal_age = false;
    }

    println!(
        "You are {}old enough to legally drive.",
        if is_legal_age {""} else {"not "}
    );
}
