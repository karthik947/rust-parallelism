use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
enum Command {
    Work(u32),
    Stop,
}

pub fn demo() {
    let (tx, rx) = mpsc::channel::<Command>();

    for job_id in 0..3 {
        tx.send(Command::Work(job_id)).unwrap();
    }
    tx.send(Command::Stop).unwrap();
    drop(tx);

    let worker = thread::spawn(move || {
        for command in rx {
            match command {
                Command::Work(id) => {
                    println!("[control] worker running job {id}");
                    thread::sleep(Duration::from_millis(30));
                }
                Command::Stop => {
                    println!("[control] received stop signal");
                    break;
                }
            }
        }
    });

    worker.join().unwrap();
}
