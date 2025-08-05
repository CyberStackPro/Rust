use std::thread;
use std::time::Duration;

fn main() {
    println!("☕ Coffee shop is open!");

    // 🧍 Main thread  takes the orders
    println!("🧍 Taking orders...");

    // 👷 Spawn a new thread: Barista makes  coffee
    let barista = thread::spawn(|| {
        println!("👷 Barista is making coffee...");
        thread::sleep(Duration::from_secs(2));
        println!("✅ Coffee is ready!");
    });

    // 👷 Spawn another thread: Waiter serves pastries
    let waiter = thread::spawn(|| {
        println!("👷 Waiter is serving pastries...");
        thread::sleep(Duration::from_secs(1));
        println!("✅ Pastries served!");
    });

    // 🧍 Meanwhile, you take more orders
    thread::sleep(Duration::from_millis(500));
    println!("🧍 Taking more orders...");

    // 🧍 You wait for both tasks to finish
    barista.join().unwrap();
    waiter.join().unwrap();

    println!("☕ Coffee shop closed. All tasks done!");
}

// ## 🧠 What’s Happening

// | Code Line                      | Real-Life Meaning                               |
// | ------------------------------ | ----------------------------------------------- |
// | `thread::spawn(...)`           | Call your friends to do tasks                   |
// | `thread::sleep(Duration::...)` | Simulate time delay (like real tasks take time) |
// | `.join()`                      | Wait for your friends to finish work            |

// ---

// ## 🖨 Output Example

// ```text
// ☕ Coffee shop is open!
// 🧍 Taking orders...
// 👷 Barista is making coffee...
// 👷 Waiter is serving pastries...
// 🧍 Taking more orders...
// ✅ Pastries served!
// ✅ Coffee is ready!
// ☕ Coffee shop closed. All tasks done!

// use std::{thread, time};

// let ten_millis = time::Duration::from_millis(10);
// let now = time::Instant::now();

// thread::sleep(ten_millis);

// assert!(now.elapsed() >= ten_millis);
