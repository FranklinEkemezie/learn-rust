
fn main() {
    let s = "hello, world".to_string();
    // or use &s
    let s1: &str = s.as_str();

    println!("Success!");
}