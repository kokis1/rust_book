use std::thread;
use std::time::Duration;
use trpl::Either;

// introducing a slow funciton so show what starving tasks looks like
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("{name} ran for {ms}ms");
}

// building a custom feature
async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration
) -> Result<F::Output, Duration> {
    match trpl::select(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}

fn main() {
    trpl::block_on( async {
        let a = async {
            println!("'a' started");
            slow("a", 30);
            trpl::yield_now().await;    // hands back control to the runtime
            slow("a", 10);              // with these yields, both functions are completed concurrently
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("a is finished");
        };

        let b = async {
            println!("'b' started");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("b is finished");
        };

        trpl::select(a, b).await;


        // testing the timeout function

        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_secs(6)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }

    })
}
