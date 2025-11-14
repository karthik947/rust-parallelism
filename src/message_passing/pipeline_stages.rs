use std::sync::mpsc;
use std::thread;

pub fn demo() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let (tx3, rx3) = mpsc::channel();

    let stage1 = thread::spawn(move || {
        for value in rx1 {
            tx2.send(format!("{value} transformed to 🍠|")).unwrap(); // double the raw value
        }
    });

    let stage2 = thread::spawn(move || {
        for value in rx2 {
            tx3.send(format!("{value} transformed to 🍟|")).unwrap();
        }
    });

    for n in 1..=3 {
        tx1.send(format!("🥔 batch{n}|")).unwrap();
    }
    drop(tx1);

    for message in rx3 {
        println!("[pipeline] {message}");
    }

    stage1.join().unwrap();
    stage2.join().unwrap();
}
