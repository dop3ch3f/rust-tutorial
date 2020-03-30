// Visualizing lifetimes

// Examples of invalid references

// Returning a reference to a value created within an inner scope

// fn main() {
//     let first_two = {
//         let list = vec![100, 34, 72, 55];
//         &list[0..2]
//     };

//     println!("First two are {:?}", first_two);
// }
 

// Returning a reference to a value created within a function

// fn main() {
//     let first_two = return_first_two();
//     println!("First two are {:?}", first_two );
// }

// fn return_first_two() -> &[i32] {
//     let list = vec![100, 34, 72, 55];
//     &list[0..2]
// }

// Returning a moved value

// fn main() {
//     let list_a = vec![100, 34, 72, 55];
//     let list_b = list_a;
//     let first_two = &list_a[0..2];
//     println!("First two are {:?}", first_two );
// }

// Self referential struct

// struct ListAndRef {
//     list: Vec<i32>,
//     first_two: &[i32],
// }

// fn return_list_and_first_two() -> ListAndRef {
//     let list_to_use = vec![100, 34, 72, 55];

//     ListAndRef {
//         list: list_to_use,
//         first_two: &list_to_use[0..2],
//     }
// }

// to enforce self referential structs checkout crates
// rental, owning-ref

// Storing references in a HashMap

// use std::io;
// use std::collections::HashMap;

// fn main() {
//     println!("Please enter some text to get word counts");
//     let mut counts = HashMap::new();
//     let mut input = String::new();

//     io::stdin().read_line(&mut input).expect("Problem reading input");

//     for word in input.split_whitespace() {
//         let count = counts.entry(word).or_insert(0);
//         *count += 1;
//     }
//     println!("counts = {:?}", counts );
// }