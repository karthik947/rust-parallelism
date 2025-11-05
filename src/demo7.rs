use std::thread::spawn;

fn task(limit: u64) -> Result<u64, String> {
    if limit > 500_000_000 {
        return Err(format!("limit {limit} is too large for this task"));
    }

    let mut x = 0_u64;
    for i in 0..=limit {
        x += i;
    }
    println!("task complete!\n");
    Ok(x)
}

pub fn demo() {
    println!("spawning cpu task!\n");
    let limit = 500_000_001_u64;
    let th = spawn(move || task(limit));
    println!("waiting for the task to complete!\n");
    match th.join() {
        Ok(Ok(total)) => println!("task completed with result {total}"),
        Ok(Err(err)) => eprintln!("task failed: {err}"),
        Err(err) => eprintln!("task panicked before returning a result: {err:?}"),
    }
}
