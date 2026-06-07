use std::thread;
use std::time::Duration;

fn main() {
    // spawning a new thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi the number {i} from the spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // if the handle.join().unwrap() was here, the thread would be forced to finish before any other code is run
    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi the number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }

    // using the .join() method to ensure the thread finishes
    handle.join().unwrap();

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || { // move is needed to ensure the closure owns all the values within it
        println!("Here's a vector: {v:?}"); // this is because v could be dropped before the thread has finished
    });

    handle.join().unwrap();
}
