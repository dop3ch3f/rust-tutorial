// entry point for all programs
fn main() {
    // add mut to make it mutable
    let mut x = 5;
    x += 1;
    // variables by default are constants
    let y = 6;
    let z = x + y;
    // heres how you print out
    println!("z is  {}", z);
}
