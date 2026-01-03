
// Mutability can be changed when ownership is transferred

fn main() {
    let s = String::from("hello, ");

    // Modify this line only !
    let mut s1 = s;     // we can make s1 mutable even if it was not originally mutable
                                // since it is now the current and the only owner of the data

    s1.push_str("world");

    println!("Success!");
}