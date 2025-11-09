use std::sync::{Arc, Mutex, TryLockError};
use std::thread::{self, spawn};
use std::time::Duration;

pub fn demo() {
    let state = Arc::new(Mutex::new(0_u32));
    let mut handles = Vec::new();

    // A single reader that periodically tries to look at the state without blocking.
    let reader_state = Arc::clone(&state);
    let reader = spawn(move || {
        for _ in 0..20 {
            match reader_state.try_lock() {
                Ok(guard) => println!("reader saw state = {guard}"),
                Err(TryLockError::WouldBlock) => println!("reader: lock busy, retrying..."),
                Err(err) => panic!("reader mutex error: {err}"),
            }
            thread::sleep(Duration::from_millis(70));
        }
    });
    handles.push(reader);

    // Multiple writers competing for exclusive access to mutate the state.
    for writer_id in 0..4 {
        let shared = Arc::clone(&state);
        let handle = spawn(move || {
            for step in 0..3 {
                thread::sleep(Duration::from_millis(60 * (writer_id + 1) as u64));
                let mut guard = shared.lock().expect("mutex lock panicked");
                *guard += 1;
                println!(
                    "writer {writer_id} step {step}: state mutated to {}",
                    *guard
                );
                thread::sleep(Duration::from_millis(30));
            }
            println!("writer {writer_id} finished");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("thread join failed");
    }
}

// pub fn demo() {
//     let mut state = Arc::new(Mutex::new(String::from("This is a ")));

//     let mut handles = Vec::new();

//     let shared1 = Arc::clone(&state);
//     let h1 = spawn(move || {
//         let mut borrowed_state = shared1.lock().expect("mutex lock paniced!");
//         borrowed_state.push_str(" broken");
//         println!("String Value => {}", borrowed_state);
//     });
//     handles.push(h1);

//     let shared2 = Arc::clone(&state);
//     let h2 = spawn(move || {
//         let mut borrowed_state = shared2.lock().expect("mutex lock paniced!");
//         borrowed_state.push_str(" string");
//         println!("String Value => {}", borrowed_state);
//     });
//     handles.push(h2);

//     for handle in handles {
//         handle.join();
//     }
// }
