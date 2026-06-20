use trpl::StreamExt;

fn main() {
    // streams are asynchronous iterators
    trpl::block_on(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);

        // iterators can be turned into streams
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was {value}");
        }
    })
}
