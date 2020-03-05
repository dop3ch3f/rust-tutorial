// Ownership

// each piece of data has one owning variable
// owner is responsible for cleaning up that data
// cleanup happens when the owner goes out of scope
// the owner decides on the mutablilty of the value

fn pluralize(value: String) -> String {
    value + "s"
}

fn main() {
    let s = String::from("book");
    println!("i have one {}, you have two {}", s, pluralize(s.clone()));
}
