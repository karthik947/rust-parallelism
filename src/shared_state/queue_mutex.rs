use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn demo() {
    let log = Arc::new(Mutex::new(VecDeque::new()));
    let mut handles = Vec::new();

    for worker_id in 0..4 {
        let shared = Arc::clone(&log);
        handles.push(thread::spawn(move || {
            let entry = format!("event from worker {worker_id}");
            let mut queue = shared.lock().expect("lock poisoned");
            queue.push_back(entry);
        }));
    }

    for handle in handles {
        handle.join().expect("thread panicked");
    }

    let mut queue = log.lock().expect("lock poisoned");
    while let Some(entry) = queue.pop_front() {
        println!("log entry: {entry}");
    }
}
