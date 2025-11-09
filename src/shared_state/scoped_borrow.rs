use std::sync::Mutex;
use std::thread;

pub fn demo() {
    let numbers = Mutex::new(vec![1, 2, 3]);

    thread::scope(|scope| {
        scope.spawn(|| {
            let mut data = numbers.lock().expect("lock poisoned");
            data.push(4);
            println!("producer pushed new value");
        });

        scope.spawn(|| {
            let sum: i32 = numbers.lock().expect("lock poisoned").iter().copied().sum();
            println!("consumer computed running sum = {sum}");
        });
    });

    {
        println!("All the scope threads have been executed!");
        let final_state = numbers.lock().expect("lock poisoned");
        println!("numbers after scope: {:?}", *final_state);
    }

    let final_state1 = numbers.lock().expect("lock poisoned");
    println!("numbers after scope: {:?}", *final_state1);
}
