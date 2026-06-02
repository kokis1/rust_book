#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // a closure can take a referene to an environment variable
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // making an iterable, nothing has happened yet

    // we can use this iterable in a loop
    for v in v1_iter {
        println!("Got: {v}");
    }

    let mut v1_iter = v1.iter();

    // calling the next() method on iterators
    // note that the next() method consumes the iterable, meaning it changes it
    // such methods are called consumer adapters
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None); // all these tests will pass

    // the .sum() method on an iterator takes ownership and consumes it
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);

    // some iterator methods don't consume the iterator, but produce another one
    // these are called iterator adapters
    let v2: Vec<i32> = vec![1, 2, 3];
    
    // the map iterator uses a closure to map each element of an iterator
    // this produces another iterator
    let second_iterable = v2.iter().map(|x| x + 1); // will produce a warning (let _ = v2.iter().map(|x| x + 1);)
                              // this is because the iterator is never calles, so is an unused variable
    
    // a fix is to use the .collect() method to produce a collection
    let v3: Vec<_> = second_iterable.collect(); // it has been 'collected' into a Vec
    
    println!("before mapping: {v2:?}");
    println!("result after mapping: {v3:?}");

}
