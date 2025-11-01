
fn main() {
    let (x, y);

    (x, ..) = (3, 4);
    [.., y] = [1, 2];

    // fill in the blank to make the code work
    assert_eq!([x, y], [3, 2]);

    println!("Success!");
}