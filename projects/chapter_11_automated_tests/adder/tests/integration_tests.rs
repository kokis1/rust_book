use adder::add_two;

#[test]
fn it_adds_two_integrate() {
    let result = add_two(2);
    assert_eq!(result, 4);
}