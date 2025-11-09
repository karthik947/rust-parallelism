use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

pub fn demo() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));

    let consumer = {
        let shared = Arc::clone(&pair);
        thread::spawn(move || {
            let (lock, condvar) = &*shared;
            let mut ready = lock.lock().expect("lock poisoned");
            while !*ready {
                ready = condvar.wait(ready).expect("lock poisoned");
            }
            println!("consumer observed the state change");
        })
    };

    let producer = {
        let shared = Arc::clone(&pair);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            let (lock, condvar) = &*shared;
            let mut ready = lock.lock().expect("lock poisoned");
            *ready = true;
            condvar.notify_all();
            println!("producer set the state and notified");
        })
    };

    consumer.join().expect("consumer panicked");
    producer.join().expect("producer panicked");
}
