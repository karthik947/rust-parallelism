use std::sync::mpsc;
use std::thread::{self, spawn};
use std::time::Duration;

pub fn demo() {
    let (tx, rx) = mpsc::sync_channel::<String>(1);

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
    thread::sleep(Duration::from_secs(2));

    while let Ok(msg) = rx.recv() {
        println!("received msg : {msg}");
        thread::sleep(Duration::from_secs(1));
    }

    println!("All channels are closed!");
}
