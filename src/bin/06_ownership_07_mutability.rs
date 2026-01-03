
fn main() {

    let x = Box::new(5);

    let mut y: Box<i32> = Box::new(1);     // implement this line, don't change other lines!

    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!");
}