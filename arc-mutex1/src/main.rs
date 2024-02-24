use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;

            let c_text = match i {
                0 => "first",
                1 => "second",
                2 => "third",
                3 => "fourth",
                4 => "fifth",
                _ => "unknown",
            };

            println!("{} thread: {}", c_text, *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_value = counter.lock().unwrap();
    println!("Final value: {}", *final_value);

    // Result: 11
    // The result is 11 because the Mutex is used to synchronize access to the data.
    // The Mutex is locked before the thread modifies the data and unlocked after the modification.
    // This ensures that only one thread can access the data at a time.
    // The Arc is used to share the Mutex between threads.
    // The Arc is a reference-counted smart pointer that allows the Mutex to be shared between threads.
    // The Arc is cloned and moved into each thread, so that each thread has a reference to the Mutex.
    // The Mutex is then locked and unlocked to access the data.
    //

}
