
// Comment one line to make it work
fn main() {

    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");


    // Turns out you can *have* many mutable reference as long as
    // you always use the latest one. Hence, I added these:

    let r3 = &mut s;
    r3.push_str("hello");

    let r4 = &mut s;
    r4.push_str("world");

    println!("{}", r4);
}