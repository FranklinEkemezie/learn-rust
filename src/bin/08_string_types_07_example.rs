
// Fix error with at least two solutions
fn main() {
    let s = "hello, world";
    // another way is to use:
    // String::from(s)
    greetings(s.to_string())

}

fn greetings(s: String) {
    println!("{}", s);
}