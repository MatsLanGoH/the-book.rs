fn main() {
    // reference to an object instead of taking ownership
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("length of {} is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
