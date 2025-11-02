
// Goal: Identify the floating point precision problem

// Make it work in two distinct ways

fn main() {
    // here 0.1 + 0.2 could be equals to something like 0.300000000000000002
    // the default float type `f64` is 'too precise' for us here
    // we can solve this by specifying a type with lower precision, `f_32` for example.
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);

    println!("Success!");
}