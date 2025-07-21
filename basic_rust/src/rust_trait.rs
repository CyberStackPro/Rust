/*
dyn ->   means = It stores a vtable (a table of function pointers) at runtime to know which method to call.
What is a Concrete Type in Rust? ->  A concrete type is a specific, fully-known type at compile time, like: i32, String, Vec<u8>
Your own types: Tweet, NewsArticle, etc.
So when we say "concrete type", we mean not a trait, and not an abstraction. It’s the actual struct or enum behind the scenes.
The * is the dereference operator in Rust. It means:
"Give me the value inside the reference."
*/
trait Draw {
    fn draw(&self);
}

struct Button {
    id: i8,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a Button with ID: {}", self.id);
    }
}

struct SelectBox {
    width: u32,
    height: u32,
}
impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "Drawing a SelectBox of size {} X {}",
            self.width, self.height
        );
    }
}

// A screen that can hold a collection of drawable objects
struct Screen {
    // `Box<dyn Draw>` means a Box containing *any* type that implements the `Draw` trait.
    // The size of `Box<dyn Draw>` is known at compile time (it's just a pointer and vtable pointer).
    // The actual type (Button or SelectBox) is only known at runtime.
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_default(&self) -> String {
        String::from("(Read more...)")
    }

    fn get_trait_version() -> String {
        String::from("1.0")
    }
}

enum Expression {
    Literal(i32),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
}

impl Expression {
    /*
       Multiply
       ├── Add
       │   ├── Literal(3) => 3
       │   └── Literal(4) => 4
       │   => 7
       └── Subtract
           ├── Literal(10) => 10
           └── Literal(5) => 5
           => 5
       => Multiply(7, 5) => 35

    */
    fn evaluate(&self) -> i32 {
        match &self {
            Expression::Literal(val) => *val,
            Expression::Add(left, right) => left.evaluate() + right.evaluate(),
            Expression::Multiply(left, right) => left.evaluate() * right.evaluate(),
            Expression::Subtract(left, right) => left.evaluate() - right.evaluate(),
        }
    }
    fn to_string(&self) -> String {
        match self {
            Expression::Literal(val) => val.to_string(),
            Expression::Add(left, right) => {
                format!("({} + {})", left.to_string(), right.to_string())
            }
            Expression::Subtract(left, right) => {
                format!("({} - {})", left.to_string(), right.to_string())
            }
            Expression::Multiply(left, right) => {
                format!("({} * {})", left.to_string(), right.to_string())
            }
        }
    }
    // fn evaluate(&self) -> i32 {
    //     match &self {
    //         Expression::Literal(val) => *val,
    //         Expression::Add(left, right) => {
    //             let l = left.evaluate();
    //             let r = right.evaluate();
    //             println!("Add: {} + {} = {}", l, r, l + r);
    //             l + r
    //         }
    //         Expression::Subtract(left, right) => {
    //             let l = left.evaluate();
    //             let r = right.evaluate();
    //             println!("Subtract: {} - {} = {}", l, r, l - r);
    //             l - r
    //         }
    //         Expression::Multiply(left, right) => {
    //             let l = left.evaluate();
    //             let r = right.evaluate();
    //             println!("Multiply: {} * {} = {}", l, r, l * r);
    //             l * r
    //         }
    //     }
    // }
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
    // let s = String::from("hi");
    // println!("{}", &s);

    // let tweet = Tweet {
    //     username: String::from("horse_ebooks"),
    //     content: String::from("of course, as you probably already know, people"),
    //     reply: false,
    //     retweet: false,
    // };
    // let article = NewsArticle {
    //     headline: String::from("Penguins win the Stanley Cup!"),
    //     location: String::from("Pittsburgh, PA"),
    //     author: String::from("Iceburgh"),
    //     content: String::from("The Pittsburgh Penguins once again won the Stanley Cup."),
    // };

    let screen = Screen {
        components: vec![
            Box::new(Button { id: 1 }),
            Box::new(SelectBox {
                width: 75,
                height: 10,
            }),
        ],
    };

    println!("\n--- Drawing Screen Components ---");
    screen.run();

    // Using Dynamic dispatch:  but this won't work b/c on our Summary trait we defined get_trait_version() is a static method ⇒ trait not object-safe ⇒ error
    // let items: Vec<Box<dyn Summary>> = vec![
    //     Box::new(Tweet {
    //         username: String::from("horse_ebooks"),
    //         content: String::from("hello world"),
    //         reply: false,
    //         retweet: false,
    //     }),
    //     Box::new(NewsArticle {
    //         headline: String::from("Rust 2.0 Released!"),
    //         location: String::from("Internet"),
    //         author: String::from("Ferris"),
    //         content: String::from("Exciting news..."),
    //     }),
    // ];

    // println!("Tweet summary: {}", tweet.summarize()); // Output: @horse_ebooks: of course, as you probably already know, people
    // println!("Article summary: {}", article.summarize()); // Output: Penguins win the Stanley Cup!, by Iceburgh (Pittsburgh, PA)

    // println!("Tweet default summary: {}", tweet.summarize_default()); // Output: (Read more...)
    // println!("Article default summary: {}", article.summarize_default()); // Output: ARTICLE: Penguins win the Stanley Cup!

    // // Call the associated function from the trait
    // println!(
    //     "Summary trait version: {}",
    //     <Tweet as Summary>::get_trait_version()
    // );

    // let largest_num = largest_i32(&[1, 20, 3, 4, 5, 6]);

    // let sentence = "hello are you fine yeabsra";
    // let chars: Vec<char> = sentence.chars().collect();

    // let largest_char = largest_char(&chars);

    // println!("Result {}", largest_char);
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
