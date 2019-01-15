fn main() {
    // reference to an object instead of taking ownership
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("length of {} is {}", s1, len);

    // The Slice Type
    let s = String::from("hello world");
    let hello = &s[0..5];  // excluding end
    let world = &s[6..=10];  // including end
    println!("{} {}", hello, world);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
