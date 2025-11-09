use std::thread::spawn;

fn task() {
    println!("This is running inside the thread \n")
}

pub fn demo() {
    println!("spawning cpu task!\n");
    let mut threads = Vec::new();

    let thread_count = 10;
    for _ in 0..thread_count {
        threads.push(spawn(task));
    }
    println!("waiting for the task to complete....\n");

    for handle in threads {
        handle.join();
    }
    println!("all threads closed");
}
