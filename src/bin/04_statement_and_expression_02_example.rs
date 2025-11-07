
// Make it work in two ways

fn main() {

    let v = {
        let mut x = 1;
        x += 2
    };

    assert_eq!(v, 2);

    println!("Success!");
}