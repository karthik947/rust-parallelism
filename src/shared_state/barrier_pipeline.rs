use std::sync::{Arc, Barrier, Mutex};
use std::thread;

pub fn demo() {
    let barrier = Arc::new(Barrier::new(3));
    let data = Arc::new(Mutex::new(Vec::<String>::new()));

    let phase1 = {
        let barrier = Arc::clone(&barrier);
        let data = Arc::clone(&data);
        thread::spawn(move || {
            {
                let mut buffer = data.lock().expect("lock poisoned");
                buffer.push("hello".to_string());
            }
            println!("phase1 inserted 'hello' into the buffer");
            barrier.wait(); // let transformer know the payload exists
            barrier.wait(); // wait until consumer finishes
            println!("phase1 observed pipeline completion");
        })
    };

    let phase2 = {
        let barrier = Arc::clone(&barrier);
        let data = Arc::clone(&data);
        thread::spawn(move || {
            barrier.wait(); // wait for producer
            {
                let mut buffer = data.lock().expect("lock poisoned");
                if let Some(item) = buffer.first_mut() {
                    item.push_str(" world");
                }
            }
            println!("phase2 appended ' world' to the payload");
            barrier.wait(); // signal consumer to proceed
        })
    };

    let phase3 = {
        let barrier = Arc::clone(&barrier);
        let data = Arc::clone(&data);
        thread::spawn(move || {
            barrier.wait(); // synchronize with producer + transformer
            barrier.wait(); // wait for transformation to finish
            let mut buffer = data.lock().expect("lock poisoned");
            if let Some(item) = buffer.first_mut() {
                item.push_str("!");
            }
            if let Some(item) = buffer.first() {
                println!("phase3 appended '!' and saw final payload: {item}");
            }
        })
    };

    phase1.join().expect("phase1 panicked");
    phase2.join().expect("phase2 panicked");
    phase3.join().expect("phase3 panicked");
}
