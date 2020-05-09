use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let mut sum = 0;
        for i in 1..10 {
            println!("thread: {}", i);
            sum += i;
            thread::sleep(Duration::from_millis(1));
        }
        tx.send(sum).unwrap();
    });

    for i in 1..5 {
        println!("out: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    let received = rx.recv().unwrap();
    println!("{}", received);
}
