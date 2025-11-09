use std::sync::{Arc, OnceLock};
use std::thread;
use std::time::Duration;

pub fn demo() {
    let shared_config = Arc::new(OnceLock::new());
    let mut handles = Vec::new();

    // First worker tries to set the config explicitly.
    {
        let config = Arc::clone(&shared_config);
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(2000));
            if config.set("lazy-config".to_string()).is_ok() {
                println!("worker 0 set the config");
            } else {
                println!("worker 0: config already initialized");
            }
        }));
    }

    // Remaining workers repeatedly read the config to show it never changes.
    for worker_id in 1..3 {
        let config = Arc::clone(&shared_config);
        handles.push(thread::spawn(move || {
            for iteration in 0..30 {
                match config.get() {
                    Some(value) => {
                        println!("worker {worker_id} on step {iteration} found value {value}")
                    }
                    None => println!("worker {worker_id} on step {iteration} found no value"),
                }
                thread::sleep(Duration::from_millis(100));
            }
        }));
    }

    for handle in handles {
        handle.join().expect("thread panicked");
    }
}
// pub fn demo() {
//     let shared_config = Arc::new(OnceLock::new());
//     let mut handles = Vec::new();

//     for worker_id in 0..3 {
//         let config = Arc::clone(&shared_config);
//         handles.push(thread::spawn(move || {
//             let value = config.get_or_init(|| {
//                 println!("worker {worker_id} performing initialization");
//                 "lazy-config".to_string()
//             });
//             println!("worker {worker_id} sees config = {value}");
//         }));
//     }

//     for handle in handles {
//         handle.join().expect("thread panicked");
//     }
// }
