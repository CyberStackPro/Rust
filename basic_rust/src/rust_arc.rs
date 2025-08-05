// Arc` stands for **"Atomic Reference Count."** It is a smart pointer that provides **thread-safe multiple ownership** of data on the heap.
use std::sync::{Arc, Mutex};
use std::thread;

// Arc
// fn main() {
//     println!("\n--- Arc<T> for Concurrent Multiple Ownership ---");

//     let shared_data = Arc::new(String::from("This data is shared across threads!"));
//     // let _second_shared = Arc::strong_count(&shared_data);
//     // let _third_shared = Arc::strong_count(&shared_data);

//     println!("Initial Arc count: {}", Arc::strong_count(&shared_data));

//     let mut handles = vec![];

//     for i in 0..3 {
//         let data_for_thread = Arc::clone(&shared_data);

//         println!(
//             "Arc count after cloning for thread {}: {}",
//             i,
//             Arc::strong_count(&shared_data)
//         );

//         let handle = thread::spawn(move || {
//             println!("Thread {} received: {}", i, data_for_thread);
//         });

//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Final Arc count: {}", Arc::strong_count(&shared_data));

//     println!("Main Thread still has access to: {}", shared_data);
// }

/*
| Concept         | Meaning                                                                   |
| --------------- | ------------------------------------------------------------------------- |
| `Arc<T>`        | Smart pointer for **thread-safe shared ownership**                        |
| `Arc::clone()`  | Increases reference count, shares ownership (doesn’t clone data)          |
| `strong_count`  | Tells how many live references exist                                      |
| `thread::spawn` | Starts a new thread, data must be `Send` and `Sync` (Arc helps with that) |

 */

//  Arc<Mutex>
fn main() {
    println!("\n--- Arc<Mutex<T>> for Shared Mutable State ---");

    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for i in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();

            *num += 1;

            println!("Thread {} incremented. Current count: {}", i, num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("\nFinal counter value: {}", *counter.lock().unwrap());
}
