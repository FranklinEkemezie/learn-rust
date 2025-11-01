
// Modify `assert_eq!` to make the `println!` print `42` in the terminal
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 5)
    }

    assert_eq!(x, 12);

    let x = 42;
    println!("{}", x);
}