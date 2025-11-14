use std::sync::mpsc;
use std::thread::spawn;

#[derive(Debug)]
struct Mango {
    layer: u32,
    color: String,
}

pub fn demo() {
    let (tx, rx) = mpsc::channel::<Mango>();

    spawn(move || {
        for i in 1..=10 {
            let data = Mango {
                layer: i,
                color: "orange".into(),
            };
            tx.send(data).unwrap();
        }
    });

    while let Ok(msg) = rx.recv() {
        println!("received msg : {:?}", msg);
    }

    println!("All channels are closed!");
}
