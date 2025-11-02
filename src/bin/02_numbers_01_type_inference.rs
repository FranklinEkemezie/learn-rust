// Type inference

// Remove something to make it work
fn main() {

    let x: i32 = 5;
    let mut y = 5; // omit type so the compiler infers the same `i32` default integer type

    y = x;

    let z = 10; // type of z? default integer type - `i32`

    println!("Success!");
}