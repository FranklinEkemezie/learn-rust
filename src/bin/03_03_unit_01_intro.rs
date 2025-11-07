
// Make it work, don't modify `implicitly_ret_unit`!

fn main() {

    // In Rust, a unit type denoted by `()` is a type that
    // does not hold any value. This is usually the return
    // type of a function that does not return any value.
    // The size of a unit type is zero.


    let _v: () = (); // a tuple with no values

    let v = (2, 3); // a tuple with two values

    assert_eq!(_v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}