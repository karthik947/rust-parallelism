use std::thread::spawn;

fn task(limit: u64) -> u64 {
    let mut x = 0_u64;
    for i in 0..=limit {
        x += i;
    }
    println!("task complete!\n");
    x
}

pub fn demo() {
    println!("spawning cpu task!\n");
    let limit = 500_000_000_u64;
    let th = spawn(move || task(limit));
    println!("waiting for the task to complete!\n");
    let total = th.join().expect("task panicked");
    println!("The total value is {}", total);
}
