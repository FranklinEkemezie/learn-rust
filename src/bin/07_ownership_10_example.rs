
fn main() {

    let t = (String::from("hello"), String::from("hello"));

    // Fill the blanks
    let (s1, s2) = t.clone();

    println!("{:?}, {:?}, {:?}", s1, s2, t);

}