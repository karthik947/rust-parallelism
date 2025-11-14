use std::sync::mpsc;
use std::thread::spawn;

pub fn demo() {
    let (tx, rx) = mpsc::channel();

    for i in 1..=3 {
        let tx_clone = tx.clone();
        spawn(move || {
            for ii in 1..=10 {
                println!("sending thread{i} message{ii}");
                tx_clone.send(format!("thread{i} message{ii}")).unwrap();
                println!("thread{i} message{ii} sent!");
            }
        });
    }
    drop(tx);

    while let Ok(msg) = rx.recv() {
        println!("received msg : {msg}");
    }

    println!("All channels are closed!");
}
