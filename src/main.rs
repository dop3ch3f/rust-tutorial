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

    // Booleans
    let a = true;
    if a {
        println!("true");
    }
    // array
    let a = [100, 200, 300];
    // usize
    let b = a[0];
    // f64 for devs
    let x = 54.3;
    let p = 'a';
    // tuples
    let tup = (1, 'c', true);
    // accessing tuples
    println!("{}", tup.0);
    // destructuring tuples
    let (x, y, z) = tup;
}
