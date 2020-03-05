use std::io;

// entry point for all programs
fn main() {
    next_birthday("ifeanyi ibekie", 33);
    println!("The square of 3 is {}", square(3));
    discount(10);
    // loop
    loop {
        println!("Whats the secret word?");
        let mut word = String::new();
        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read Line");

        if word.trim() == "rust" {
            println!("Genius youve got it");
            break;
        }
    }

    // while example
    let mut word = String::new();
    while word.trim() != "rust" {
        println!("Whats the secret word?");
        word = String::new();
        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read Line");
    }
    // for loops
    for i in 1..3 {
        println!("Now serving number {}", i);
    }

    // match
    let x = 3;
    match x {
        1 => println!("matched one"),
        2 => println!("matched two"),
        3 => println!("matched three"),
        _ => println!("matched another number"),
    }

    // concrete match example
    let die1 = 1;
    let die2 = 5;

    match (die1, die2) {
        (1,1) => println!("Go to begining"),
        (5, _) | (_, 5) => {
            println!("you rolled atleast a 5");
        },
        _ => println!("you can move")
    }

    // exhaustive checking rust example

    let is_confirmed = true;
    let is_active = false;

    match (is_confirmed, is_active) {
        (true, true) => println!("this account is in good standing"),
        (false, true) => println!("you will need to confirm your account"),(false, false) => println!("this account will be deactivated"),
        (true, false) => println!("this is the final case and you are active but not confirmed"),
    }
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

fn discount(day_of_the_month: u8) {
    let amount = if day_of_the_month % 2 == 0 { 50 } else { 10 };

    println!("Your discount is {}%", amount);
}
