use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
struct LogEntry {
    level: String,
    message: String,
    thread_id: u64,
}

fn main() {
    println!("--- Concurrent Log Processor: Initializing ---");

    let log_buffer: Arc<Mutex<Vec<LogEntry>>> = Arc::new(Mutex::new(Vec::new()));

    println!(
        "Shared log buffer created. Initial Arc count: {}",
        Arc::strong_count(&log_buffer)
    );

    let mut producer_handlers = vec![];

    let num_producers = 3;

    let messages_per_producer = 5;

    for i in 0..num_producers {
        let producer_log_buffer_arc = Arc::clone(&log_buffer);

        let thread_id = i;

        let handle = thread::spawn(move || {
            println!("[Producer {}] Starting...", thread_id);
            for j in 0..messages_per_producer {
                let level = if j % 2 == 0 { "INFO" } else { "DEBUG" }.to_string();

                let message_count = format!("Thread {} - Message {}", thread_id, i);

                let log_entry = LogEntry {
                    level,
                    message: message_count,
                    thread_id,
                };

                let mut buffer_guard = producer_log_buffer_arc.lock().unwrap();

                buffer_guard.push(log_entry);
                println!(
                    "[Producer {}] Added log. Buffer size: {}",
                    thread_id,
                    buffer_guard.len()
                );

                thread::sleep(Duration::from_millis(100));
            }

            println!("[Producer {}] Finished generating logs.", thread_id);
        });

        producer_handlers.push(handle);
    }

    for handle in producer_handlers {
        handle.join().expect("Producer thread panicked!");
    }

    println!("\n--- All Producer Threads Finished ---");
    println!(
        "Final Arc count in main after producers: {}",
        Arc::strong_count(&log_buffer)
    );

    let final_buffer_guard = log_buffer.lock().unwrap();

    println!("\n--- Current Log Buffer Content (from Main Thread) ---");

    for entry in final_buffer_guard.iter() {
        println!(
            "[{}] Thread {}: {}",
            entry.level, entry.thread_id, entry.message
        );
    }

    println!("Total logs in buffer: {}", final_buffer_guard.len());
}
