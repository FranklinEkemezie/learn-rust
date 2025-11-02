#[allow(unused_variables)]

// Goal: Modify `assert_eq!` to make it work

fn main() {
    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of the given variable, return the string representation of the type
// e.g. i32, u64 etc.
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}