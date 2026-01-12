
// Fix all errors without adding newline
fn main() {
    let s: String = String::from("hello");
    s.push(',');
    s.push(" world");
    s += "!".to_string();

    println!("{}", s);
}