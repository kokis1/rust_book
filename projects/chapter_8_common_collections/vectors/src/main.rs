#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}


fn main() {
    // creating a new empty vector
    let mut v: Vec<i32> = Vec::new();

    // adding elements to the vector
    v.push(1);
    v.push(45);
    v.push(13);

    // creating a new vector with starting values
    let v2 = vec![1, 2, 3];

    // accessing elements of a vector
    // 1: by indexing
    let num: &i32 = &v[2]; // accessing an immutable reference to the second element of v (to maintain ownership with v)
    println!("The third element of v is {num}");

    // 2: by the .get(i) method, this provides bounds checking
    let num2: Option<&i32> = v2.get(1); // the get method returns an option to the index
    match num2 {
        None => println!("num2 has fewer than 2 elements"),
        Some(i) => println!("The second element of num2 is {i}")
    };

    // iterating over vector elements
    let v = vec![1, 2, 101];
    for i in &v {
        println!("{i}");
    }

    // iterating over immutable references to a vector
    let mut v = vec![1, 3, 4, 6];
    for i in &mut v {
        // using the dereference operator (much like in c)
        *i += 1;
        println!("Adding 1 to {} -> {i}", &*i-1);
    }

    // vector elements must all be of the same type, so use enums to hold variants
    let row = vec![
        SpreadsheetCell::Text(String::from("helllo world!")),
        SpreadsheetCell::Int(34),
        SpreadsheetCell::Float(10.5),
    ];
    println!("The row of this spreadsheet is {row:?}");


}
