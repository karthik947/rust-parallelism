use std::sync::mpsc;
use std::thread::spawn;

pub fn demo() {
    let (tx, rx) = mpsc::channel();

    spawn(move || {
        for i in 1..=10 {
            tx.send(format!("message {i} from the sender")).unwrap();
        }
    });

    while let Ok(msg) = rx.recv() {
        println!("received msg : {msg}");
    }

    println!("All channels are closed!");
}
