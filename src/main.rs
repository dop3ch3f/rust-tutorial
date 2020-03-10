// Result Type
// an enum that returns Ok or Err
// Option Type
// an enum that returns Some(if value) or None(if no value)

fn main() {
    let nonempty_list = vec!['a', 'b', 'c'];
    println!("nonempty_list last is: {:?}", nonempty_list.last());

    let empty_list: Vec<char> = vec![];
    println!("empty_list last is: {:?}", empty_list.last());
}
// What to do with Result or Option
// Success Ok() or  Some()
// to get the value of Ok()
  
// let first_inner = match first {
//     Ok(inner) => inner,
//     Err(_) => panic!(),
//     Err(error) => panic!(),
//     _ => unimplemented!(),
// };

// Converting Recoverable to Unrecoverable