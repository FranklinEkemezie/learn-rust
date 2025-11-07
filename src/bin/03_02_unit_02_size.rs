
// Modify `4` in assert to make it work

fn main() {

    let unit: () = ();
    assert!(size_of_val(&unit) == 4);

    println!("Success!");
}