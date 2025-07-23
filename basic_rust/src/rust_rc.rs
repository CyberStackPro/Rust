/* `Rc<T>` - Reference Counting for Multiple Ownership

    Rc stands for **"reference counting."**
    Rc<T>` allows for **multiple ownership** of data on the heap.
    When multiple `Rc` pointers point to the same data, `Rc<T>` keeps a count of how many active references there are.
    When the count drops to zero (meaning all `Rc` pointers to that data have gone out of scope), the data is automatically cleaned up.

*/
use std::rc::Rc;

// enum RcList {
//     Cons(i32, Rc<RcList>),
//     Nil,
// }

// use RcList::{Cons, Nil};

// fn main() {
//     println!("\n--- Rc<T> for Multiple Ownership ---");

//     let a = Rc::new(Cons(5, Rc::new(Cons(5, Rc::new(Nil)))));

//     println!("Count after creating a: {}", Rc::strong_count(&a));

//     let b = Cons(3, Rc::clone(&a));
//     println!("Count after creating b: {}", Rc::strong_count(&a));

//     {
//         let c = Cons(4, Rc::clone(&a));
//         println!("Count after creating c: {}", Rc::strong_count(&a));
//         println!("List c created.");
//     }

//     println!("Count after c goes out of scope: {}", Rc::strong_count(&a));

//     println!("List b created.");
// }

// mini project

struct AppConfig {
    log_level: String,
    api_key: String,
    timeout_seconds: u64,
}
fn module_a(config: Rc<AppConfig>) {
    println!("[Module A] Log Level: {}", config.log_level);
    println!("[Module A] Rc count: {}", Rc::strong_count(&config));
}

fn module_b(config: Rc<AppConfig>) {
    println!("[Module B] API Key: {}", config.api_key);
    println!("[Module B] Rc count: {}", Rc::strong_count(&config));
}

fn module_c(config: Rc<AppConfig>) {
    println!("[Module C] Timeout: {} seconds", config.timeout_seconds);
    println!("[Module C] Rc count: {}", Rc::strong_count(&config));
}
fn main() {
    let config = Rc::new(AppConfig {
        api_key: "123456".to_string(),
        log_level: "HIGH".to_string(),
        timeout_seconds: 6784734,
    });

    println!("[Main] Initial Rc count: {}", Rc::strong_count(&config));

    module_a(Rc::clone(&config));
    module_b(Rc::clone(&config));
    module_c(Rc::clone(&config));

    println!("[Main] Final Rc count: {}", Rc::strong_count(&config));
}
