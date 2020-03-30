// Concrete Lifetimes

// What is a concrete lifetime of values and references

// A lifetime is the time a value exists at one memory address e.g a real persons address
// Start: when a value is created or moved into a location in memory
// End: Moved or dropped from that location
// values and references are the same in operation but the added constraint for references is that their value must be container withing the referenced values's lifetime


// Visualize concrete lifetimes of values and references in code
// - within one function

// fn main() {
//     let list = vec![100,34,72,55];
//     let first_two = &list[0..2];
//     println!("first two are {:?}", first_two );
//     println!("list is {:?}", list);
//     println!("again, first two: {:?}", first_two );
// }

// - across multiple functions

fn return_first_two(borrowed_list: &[i32]) -> &[i32] {
    &borrowed_list[0..2]
}

fn main() {
    let list = vec![100,34,72, 55];
    let first_two = return_first_two(&list);
    println!("first two are {:?}", first_two );
    println!("list is {:?}", list);
    println!("again, first two: {:?}", first_two );
}