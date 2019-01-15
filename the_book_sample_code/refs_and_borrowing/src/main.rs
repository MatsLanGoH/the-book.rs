fn main() {
    // reference to an object instead of taking ownership
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("length of {} is {}", s1, len);

    // The Slice Type
    let s = String::from("hello world, how are you");
    let hello = &s[..5];  // excluding end, starting at the beginning
    let world = &s[6..=10];  // including end
    let howareyou = &s[11..]; // up to the end
    println!("{} {} {}", hello, world, howareyou);

    println!("{}", &s[..]); // this works as well!

    // Slicing an array
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{:?}", slice);
}

fn calculate_length(s: &str) -> usize {
    s.len()
}
