use std::thread::spawn;

fn task() {
    let mut x = 0_u128;
    for i in 0..=500_000_000 {
        x += i;
    }
    println!("task complete!\n");
}

pub fn demo() {
    println!("spawning cpu task!\n");
    let thread_handler1 = spawn(task);
    let thread_handler2 = spawn(task);
    println!("waiting for the task to complete....\n");

    loop {
        println!("Threads are grinding!! Chill bro 😎");
        let status1 = thread_handler1.is_finished();
        let status2 = thread_handler2.is_finished();
        if status1 && status2 {
            break;
        }
    }
    println!("all threads are closed....\n");
}
