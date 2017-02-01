use std::sync::mpsc::channel;
use std::thread;
use std::sync::{Arc, Mutex};

const NUM_VALS: u32 = 5;
fn main() {
    let counter = Arc::new(Mutex::new(0));
    let (tx, rx) = channel();
    for i in 0..NUM_VALS {
        let (counter, tx) = (counter.clone(), tx.clone());
        thread::spawn(move || {
            let mut counter = counter.lock().unwrap();
            *counter += 1;
            tx.send(i).unwrap();
        });
    }

    for _ in 0..NUM_VALS {
        print!("{} ", rx.recv().unwrap());
    }
    print!("-> num values: {}", *counter.lock().unwrap());
}