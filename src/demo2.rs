use std::thread::spawn;

fn task1() {
    println!("This is running inside the thread1 \n")
}

fn task2() {
    println!("This is running inside the thread2 \n")
}

pub fn demo() {
    println!("spawning cpu task!\n");
    let th1 = spawn(task1);
    let th2 = spawn(task2);
    println!("waiting for the task to complete....\n");
    th1.join();
    th2.join();
    println!("all threads closed");
}
