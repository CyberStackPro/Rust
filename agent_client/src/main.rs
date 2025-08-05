use std::io;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::time::Duration;

fn main() {
    let stop_signal = Arc::new(AtomicBool::new(false));
    let stop_signal_clone = Arc::clone(&stop_signal);

    let handle = thread::spawn(move || {
        println!("Worker: started");
        while !stop_signal_clone.load(Ordering::SeqCst) {
            println!("Worker: working...");
            thread::sleep(Duration::from_secs(1));
        }
        println!("Worker: received stop signal, exiting.");
    });

    println!("Press Enter to stop...");
    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).unwrap();

    println!("Main: sending stop signal.");
    stop_signal.store(true, Ordering::SeqCst);

    handle.join().unwrap();
    println!("Main: worker has stopped.");
}
