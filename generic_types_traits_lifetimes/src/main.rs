use generic_types_traits_lifetimes::{
    Tweet,
    Summary,
};

fn _main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}",
        tweet.summarize()
    );
}


fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn main() {
    let number_list = vec![1231, 12314, 91230, 1238, 123];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}