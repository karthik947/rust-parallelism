use std::thread::spawn;

fn task() {
    println!("This is running inside the thread\n")
}

pub fn demo() {
    println!("spawning cpu task!\n");
    let th = spawn(task);
    println!("waiting for the task to complete....\n");
    th.join();
    println!("all threads closed");
}
