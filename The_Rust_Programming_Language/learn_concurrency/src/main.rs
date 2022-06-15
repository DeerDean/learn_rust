use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};
use std::rc::Rc;


fn main() {
    // // thread ---------------------
    // let v = vec![1, 2, 3];
    // let handle = thread::spawn(move || {
    //     println!("Here's a vector: {:?}", v);
    // });
    // handle.join().unwrap();

    // // message passing: mpsc -------------------
    // let (tx, rx) = mpsc::channel();
    // let tx1 = tx.clone();

    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("more"),
    //         String::from("messages"),
    //         String::from("for"),
    //         String::from("you"),
    //     ];

    //     for val in vals {
    //         tx1.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // for received in rx {
    //     println!("Got: {}", received);
    // }
    // // let received = rx.recv().unwrap();
    // // println!("Got: {}", received);

    // mutex ------------------------
    let m = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let m = Arc::clone(&m);
        let handle = thread::spawn(move || {
            let mut num = m.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("m = {:?}", m);
}
