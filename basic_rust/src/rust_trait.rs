pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_default(&self) -> String {
        String::from("(Read more...)")
    }

    fn get_trait_version() -> String {
        String::from("1.0")
    }
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_default(&self) -> String {
        format!("ARTICLE: {}", self.headline)
    }
}

impl Summary for Tweet {
    fn get_trait_version() -> String {
        String::from("2.0")
    }

    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

fn main() {
    let s = String::from("hi");
    println!("{}", &s);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup!"),
        location: String::from("Pittsburgh, PA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again won the Stanley Cup."),
    };

    // println!("Tweet summary: {}", tweet.summarize()); // Output: @horse_ebooks: of course, as you probably already know, people
    // println!("Article summary: {}", article.summarize()); // Output: Penguins win the Stanley Cup!, by Iceburgh (Pittsburgh, PA)

    // println!("Tweet default summary: {}", tweet.summarize_default()); // Output: (Read more...)
    // println!("Article default summary: {}", article.summarize_default()); // Output: ARTICLE: Penguins win the Stanley Cup!

    // // Call the associated function from the trait
    // println!(
    //     "Summary trait version: {}",
    //     <Tweet as Summary>::get_trait_version()
    // );

    let largest_num = largest_i32(&[1, 20, 3, 4, 5, 6]);

    let sentence = "hello are you fine yeabsra";
    let chars: Vec<char> = sentence.chars().collect();

    let largest_char = largest_char(&chars);

    println!("Result {}", largest_char);
}

// Generics
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn add_one(n: &mut i32) {
    *n += 1;
}

fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
// let nums = vec![10, 20, 15];
// let largest_num = largest_i32(&nums); // works only for i32

// let chars = vec!['a', 'z', 'm'];
// let largest_char = largest_ref(&chars); // works for chars because char: PartialOrd

// let floats = vec![1.2, 3.4, 2.2];
// let largest_float = largest_ref(&floats); // works for floats
