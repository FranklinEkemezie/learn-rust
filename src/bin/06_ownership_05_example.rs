
// Don't use clone, use copy instead
fn main() {
    // Change the values in the tuple 'x' to types of known fixed size
    let x: (i32, i32, (), &str) = (1, 2, (), "hello");
    let y = x;

    println!("{:?}, {:?}", x, y);

}