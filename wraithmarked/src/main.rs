// trait Animal {
//     fn speak(&self) -> String;
//     fn eat(&self, food: &str);
// }

// struct Dog;
// struct Cat;

// impl Animal for Dog {
//     fn speak(&self) -> String {
//         "Woof!".to_string()
//     }
//     fn eat(&self, food: &str) {
//         println!("Dogs eats {}", food);
//     }
// }

// impl Animal for Cat {
//     fn speak(&self) -> String {
//         "Meow!".to_string()
//     }
//     fn eat(&self, food: &str) {
//         println!("Cat eats {}", food);
//     }
// }
// fn main() {
// let dog = Dog;
// let cat = Cat;

// println!("Dog says: {}", dog.speak());
// dog.eat("bone");

// println!("Cat says: {}", cat.speak());
// cat.eat("fish");

// print_twice(42);
// print_twice("hello");

// let dog = Dog;
// let cat = Cat;

// make_speak(&dog);
// make_speak(&cat);
// }
// fn print_twice<T: std::fmt::Debug>(value: T) {
//     print!("{:?} {:?}", value, value);
// }
// fn make_speak<T: Animal>(animal: &T) {
//     println!("{}", animal.speak());
// }

// #[derive(Debug)]
// struct Task {
//     id: u32,
//     title: String,
//     completed: bool,
// }

// #[derive(Debug)]
// struct ShoppingTask {
//     id: u32,
//     title: String,
//     completed: bool,
//     items: Vec<String>,
// }

// #[derive(Debug)]
// struct TimedTask {
//     id: u32,
//     title: String,
//     completed: bool,
//     deadline: String,
// }
// trait TaskBehavior {
//     fn display(&self);
//     fn toggle_complete(&mut self);
// }

// impl TaskBehavior for Task {
//     fn display(&self) {
//         println!(
//             "Task {}: {} [{}]",
//             self.id,
//             self.title,
//             if self.completed { "done" } else { "not done" }
//         );
//     }
//     fn toggle_complete(&mut self) {
//         self.completed = !self.completed;
//     }
// }

// impl TaskBehavior for ShoppingTask {
//     fn display(&self) {
//         println!(
//             "Shopping Task {}: {} [{}] -> Items: {:?}",
//             self.id,
//             self.title,
//             if self.completed { "done" } else { "not done" },
//             self.items
//         );
//     }
//     fn toggle_complete(&mut self) {
//         self.completed = !self.completed
//     }
// }

// impl ShoppingTask {
//     fn add_items(&mut self, item: String) {
//         self.items.push(item);
//     }
// }
// impl TaskBehavior for TimedTask {
//     fn display(&self) {
//         println!(
//             "TimedTask {}: {} [{}] -> Deadline: {}",
//             self.id,
//             self.title,
//             if self.completed { "done" } else { "not done" },
//             self.deadline
//         );
//     }
//     fn toggle_complete(&mut self) {
//         self.completed = !self.completed
//     }
// }

// fn complete_task<T: TaskBehavior>(task: &mut T) {
//     task.toggle_complete();
// }
// fn show_task<T: TaskBehavior>(task: &T) {
//     task.display();
// }
// fn add_items(item:String){
//     let new_item= ShoppingTask::add_items(&mut self, item);
// }
// fn main() {
//     let mut t1 = Task {
//         id: 1,
//         title: "Learn Rust".to_string(),
//         completed: false,
//     };
//     let mut t2 = ShoppingTask {
//         id: 2,
//         title: "Buy groceries".to_string(),
//         completed: false,
//         items: vec!["milk".into(), "bread".into()],
//     };
//     let mut t3 = TimedTask {
//         id: 3,
//         title: "Submit homework".to_string(),
//         completed: false,
//         deadline: "2025-09-20".to_string(),
//     };

//     show_task(&t1);
//     show_task(&t2);
//     show_task(&t3);
//     shopping.add_item("milk".to_string());

//     complete_task(&mut t2);
//     complete_task(&mut t3);

//     println!("--- After completing ---");
//     show_task(&t2);
//     show_task(&t3);
// }

trait TaskBehavior {
    fn display(&self);
    fn toggle_complete(&mut self);
}

#[derive(Debug)]
struct BasicTask {
    id: u32,
    title: String,
    completed: bool,
}

#[derive(Debug)]
struct ShoppingTask {
    id: u32,
    title: String,
    completed: bool,
    items: Vec<String>,
}

// Trait implementations
impl TaskBehavior for BasicTask {
    fn display(&self) {
        println!("Task {}: {} [{}]", self.id, self.title, self.completed);
    }
    fn toggle_complete(&mut self) {
        self.completed = !self.completed;
    }
}

impl TaskBehavior for ShoppingTask {
    fn display(&self) {
        println!(
            "Shopping {}: {} [{}] with items {:?}",
            self.id, self.title, self.completed, self.items
        );
    }
    fn toggle_complete(&mut self) {
        self.completed = !self.completed;
    }
}

// Inherent method for ShoppingTask only
impl ShoppingTask {
    fn add_item(&mut self, item: String) {
        self.items.push(item);
    }
}

fn main() {
    let mut tasks: Vec<Box<dyn TaskBehavior>> = vec![
        Box::new(BasicTask {
            id: 1,
            title: "Learn Rust".into(),
            completed: false,
        }),
        Box::new(ShoppingTask {
            id: 2,
            title: "Groceries".into(),
            completed: false,
            items: vec![],
        }),
    ];

    for task in tasks.iter_mut() {
        task.display(); // ✅ works
        task.toggle_complete(); // ✅ works
    }

    // But this DOESN’T work:
    tasks[1].add_item("milk".into());
}
