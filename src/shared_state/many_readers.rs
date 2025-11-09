use std::sync::{Arc, RwLock, TryLockError};
use std::thread;
use std::time::Duration;

pub fn demo() {
    let shared_text = Arc::new(RwLock::new(String::from("hello")));
    let mut handles = Vec::new();

    // Reader using the blocking read() method.
    let read_lock = Arc::clone(&shared_text);
    let blocking_reader = thread::spawn(move || {
        for idx in 0..4 {
            {
                let guard = read_lock.read().expect("read lock poisoned");
                println!("blocking reader {idx} saw: {guard}");
            }
            thread::sleep(Duration::from_millis(90));
        }
    });
    handles.push(blocking_reader);

    // Reader using try_read() to check the lock without blocking.
    let try_read_lock = Arc::clone(&shared_text);
    let try_reader = thread::spawn(move || {
        for _ in 0..12 {
            match try_read_lock.try_read() {
                Ok(guard) => println!("try_reader got lock: {guard}"),
                Err(TryLockError::WouldBlock) => {
                    println!("try_reader: lock busy, retrying...");
                }
                Err(err) => panic!("try_reader error: {err}"),
            }
            thread::sleep(Duration::from_millis(50));
        }
    });
    handles.push(try_reader);

    // Writer using the blocking write() method.
    let write_lock = Arc::clone(&shared_text);
    let writer = thread::spawn(move || {
        thread::sleep(Duration::from_millis(200)); // let readers run first
        let mut guard = write_lock.write().expect("write lock poisoned");
        guard.push_str(" world");
        println!("writer appended text -> {}", *guard);
        thread::sleep(Duration::from_millis(150));
    });
    handles.push(writer);

    // Writer attempting to use try_write() to avoid blocking.
    let try_write_lock = Arc::clone(&shared_text);
    let try_writer = thread::spawn(move || {
        for attempt in 0..8 {
            match try_write_lock.try_write() {
                Ok(mut guard) => {
                    guard.push_str("!");
                    println!("try_writer succeeded on attempt {attempt}: {}", *guard);
                    break;
                }
                Err(TryLockError::WouldBlock) => {
                    println!("try_writer attempt {attempt}: lock busy");
                    thread::sleep(Duration::from_millis(70));
                }
                Err(err) => panic!("try_writer error: {err}"),
            }
        }
    });
    handles.push(try_writer);

    for handle in handles {
        handle.join().expect("thread join failed");
    }
}
