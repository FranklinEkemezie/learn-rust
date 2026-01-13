
// Fix errors without removing any line
fn main() {

    let s1 = String::from("hello,");
    let s2 = String::from("world!");

    // we can also add a reference to `s2`, `&s2` instead, and
    // `&s2` will be inferred tas `String`
    let s3: String = s1 + s2.as_str();
    assert_eq!(s3, "hello,world!");
    println!("{}", s3);
}