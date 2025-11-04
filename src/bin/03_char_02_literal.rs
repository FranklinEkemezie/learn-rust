
// Make it work

fn main () {

    // In rust, double quotes are for string literals,
    // while single quotes must be used for character literals.

    let c1 = 'ϕ';
    print_char(c1);
}

fn print_char(c: char) {
    println!("{}", c);
}