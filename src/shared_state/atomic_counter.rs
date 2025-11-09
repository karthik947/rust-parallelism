use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

pub fn demo() {
    let counter = Arc::new(AtomicUsize::new(100_000));
    let mut handles = Vec::new();

    for worker_id in 0..4 {
        let shared = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1_000 {
                shared.fetch_sub(1, Ordering::Relaxed);
            }
            let current = shared.load(Ordering::Relaxed);
            println!("worker {worker_id} observed counter at {current}");
        }));
    }

    //store a value after 5 secs
    let shared = Arc::clone(&counter);
    handles.push(thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        let before_value = counter.load(Ordering::Relaxed);
        println!("before value: {before_value}");
        shared.store(78, Ordering::Relaxed);
        let after_value = counter.load(Ordering::Relaxed);
        println!("after value: {after_value}");

        match shared.compare_exchange(78, 100, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(n) => println!("Compare Exchange Success; return value {n}"),
            Err(m) => println!("Compare Exchange Error; return value {m}"),
        }
        let after_value = counter.load(Ordering::Relaxed);
        println!("value after compare exchange: {after_value}");

        match shared.compare_exchange(78, 100, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(n) => println!("Compare Exchange Success; return value {n}"),
            Err(m) => println!("Compare Exchange Error; return value {m}"),
        }
        let after_value = counter.load(Ordering::Relaxed);
        println!("value after compare exchange: {after_value}");
    }));

    for handle in handles {
        handle.join().expect("thread panicked");
    }
}
// pub fn demo() {
//     let counter = Arc::new(AtomicUsize::new(0));
//     let mut handles = Vec::new();

//     for worker_id in 0..4 {
//         let shared = Arc::clone(&counter);
//         handles.push(thread::spawn(move || {
//             for _ in 0..1_000 {
//                 shared.fetch_add(1, Ordering::Relaxed);
//             }
//             let current = shared.load(Ordering::Relaxed);
//             println!("worker {worker_id} observed counter at {current}");
//         }));
//     }

//     for handle in handles {
//         handle.join().expect("thread panicked");
//     }

//     let final_value = counter.load(Ordering::Relaxed);
//     println!("final atomic counter: {final_value}");
// }
