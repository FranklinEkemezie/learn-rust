
fn main() {

    // Another solution is to remove the `.into()` call
    // and the `Box` type so that `s` has the default `&str` type
    // In this case, we won't have to pass a reference of `s` to
    // `greetings` function since the type matches.
    let s: Box<str> = "hello, world".into();
    greetings(&s)
}

fn greetings(s: &str) {
    println!("{}", s);
}