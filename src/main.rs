use std::time::Duration;

use rand::Rng;

use hft_trading::low_latency_comm::{SPSC, Sender, Receiver};

fn main() {
    // 1. Initialize the single Queue instance.
    let queue = SPSC::<i32>::new();

    // 2. Use the split() method to break the queue into the two handles.
    // The (tx, rx) convention is standard for Sender and Receiver.
    // This consumes the 'queue' variable and strategically leaks the memory.
    let (tx, rx) = queue.split();

    // --- Thread 1: Sender (The Producer) ---
    // The 'move' keyword transfers ownership of the 'tx' handle to the new thread.

    let producer_thread = std::thread::spawn(move || {
        loop {
            let mut rng = rand::rng();
            let random = rng.random_range(1..=100);
            if tx.send(random).is_ok() {
                println!("[Sender] Sent: {}", random);
            } else {
                println!("[Sender] Queue was full.");
            }
            std::thread::sleep(Duration::from_millis(200));
        }

    });

    // --- Thread 2: Receiver (The Consumer) ---
    // The 'move' keyword transfers ownership of the 'rx' handle to the new thread.
    let consumer_thread = std::thread::spawn(move || {
        loop {
            match rx.recv() {
            Ok(item) => println!("[Receiver] Received: {}", item),
            Err(()) => continue,
        }
        }
    });

    // Wait for the threads to finish execution.
    producer_thread.join().unwrap();
    consumer_thread.join().unwrap();
}
