
// Make it  work

use std::mem::size_of_val;

fn main() {
    // In Rust, a character type ('char') takes up 4 bytes
    // It is big enough to hold any Unicode character

    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);

    let c2 = 'ϕ';
    assert_eq!(size_of_val(&c2), 4);

    println!("Success!");
}