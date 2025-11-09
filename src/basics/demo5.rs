use std::thread::spawn;

fn task(limit: u64) {
    let mut x = 0_u64;
    for i in 0..=limit {
        x += i;
    }
    println!("task complete!\n");
}

pub fn demo() {
    println!("spawning cpu task!\n");
    let limit = 500_000_000_u64;
    let th = spawn(move || task(limit));
    th.join();
}
