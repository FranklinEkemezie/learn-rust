
// Goals:
// - Modify `assert!` to make it work
// Make `println!` output: 97 - 122
fn main() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i;
    }

    assert!(sum == -3);

    for c in 'a'..='z' {
        println!("{}", c);
    }
}