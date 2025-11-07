
// Make it work in two ways

fn main() {

    let v = {
        let mut x = 1;
        x += 2; // assignment operation does not return any value
        x // return the value of x
    };

    assert_eq!(v, 3);

    println!("Success!");
}