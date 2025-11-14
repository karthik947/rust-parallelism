use std::sync::mpsc;
use std::thread;

pub fn demo() {
    let (tx, rx) = mpsc::channel::<(u32, mpsc::Sender<String>)>();

    let worker = thread::spawn(move || {
        for (input, reply_tx) in rx {
            let result = format!("{input} squared = {}", input * input);
            reply_tx.send(result).unwrap();
        }
    });
    for n in 1..=3 {
        let (reply_tx, reply_rx) = mpsc::channel();
        tx.send((n, reply_tx)).unwrap();
        let response = reply_rx.recv().unwrap();
        println!("[one-shot] got response: {response}");
    }
    drop(tx); // stop the worker
    worker.join().unwrap();
}
