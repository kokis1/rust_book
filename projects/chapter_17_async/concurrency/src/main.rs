use std::time::Duration;

fn main() {
    trpl::block_on(async {

        // spawn an async task to add another part of concurrency
        let handle = trpl::spawn_task( async {
            for i in 1..10 {
                println!("hi the number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi the number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        handle.await.unwrap();


        // the below code is completely equivalent to the above code

        let fut_1 = async {
            for i in 1..10 {
                println!("hi the number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut_2 = async {
            for i in 1..5 {
            println!("hi the number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // the futures need to be rejoined to be run

        // the one difference is that now the output is the same every time
        // under the hood, trpl checks each function just as often in the same places
        // meaning that the output is now fully deterministic
        trpl::join(fut_1, fut_2).await;


        // message passing between async tasks

        let (tx, mut rx) = trpl::channel();

        let val = String::from("hi");
        tx.send(val).unwrap();

        let recieved = rx.recv().await.unwrap();
        println!("Recieved '{recieved}'");


        // sending a series of messages

        let tx_async = async move { // the move takes ownership of tx into this block
                                    // now the tx closes and returns None at the end of this block
                                    // so the program will actually end
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500));
            }
        };

        let rx_async = async {
            while let Some(value) = rx.recv().await {
                println!("Recieved '{value}'");
            }
        };

        trpl::join(tx_async, rx_async).await;
    });
}
