// entry point for all programs
fn main() {
    next_birthday("ifeanyi ibekie", 33);
    println!("The square of 3 is {}", square(3))
}

/// fn name(param1: type1, ...) -> return_type {
///   ...body...
/// }

fn next_birthday(name: &str, current_age: u8) {
    let next_age = current_age + 1;
    println!(
        "Hi {}, on your next birthday, you will be {}",
        name, next_age
    );
}

fn square(num: isize) -> isize {
    num * num
}
