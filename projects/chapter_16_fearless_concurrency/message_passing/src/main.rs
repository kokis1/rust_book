use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // making a channel for threads to communicate with
    let (tx, rx) = mpsc::channel(); // MPSC:
                                    // MULTIPLE
                                    // PRODUCER
                                    // SINGLE
                                    // RECIEVER - one channel can recieve the messages
                                    //          - many channels can send messages
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let recieved = rx.recv().unwrap();
    println!("Got: {recieved}");

    // sending multiple messages to the same reciever
    let (tx, rx) = mpsc::channel();
    // spawning a second thread and second transmitter
    let tx1 = tx.clone();
    
    thread::spawn(move || {
        let vals = vec![
            String::from("hi (1)"),
            String::from("from (1)"),
            String::from("the (1)"),
            String::from("thread (1)"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(750));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi (2)"),
            String::from("from (2)"),
            String::from("the (2)"),
            String::from("thread (2)"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(750));
        }
    });

    for recieved in rx { // rx can be used as an iterator
        println!("Got: {recieved}"); // after each value is printed the thread waits for the next message
    }

}
