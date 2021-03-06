use generic_types_traits_lifetimes::{
    Tweet,
    Summary,
    NewsArticle,
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

// Bound Trait for generic
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    // find largest i32
    let number_list = vec![1231, 12314, 91230, 1238, 123];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    // find largest char
    let char_list = vec!['a', 'y', 'o', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // Play around with traits
    let tweet = Tweet {
        username: String::from("water_stapler"),
        content: String::from("I'm looking for a more exciting career"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        author: String::from("Kent Brockman"),
        headline: String::from("Fire in the hole"),
        content: String::from("The Springfield annual fireworks competition ends with a terrible disaster."),
        location: String::from("Springfield"),
    };

    println!("News flash: {}", article.summarize());
}