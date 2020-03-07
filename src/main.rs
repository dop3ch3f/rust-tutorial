fn main() {
    // let v = vec![10,20,30];
    // let v_slice = &v[..9];
    // println!("v_slice is: {:?}", v_slice);

    let a = [1, 2, 3];
    let v = vec![4, 5, 6];
    // how to create a slice
    let v_slice = &v[..];

    only_reference_to_array(&a);
    only_reference_to_vector(&v);
    reference_to_either_array_or_vectors(v_slice);
    reference_to_either_array_or_vectors(&a);

    let s = String::from("hi");
    let string_literal = "hello";
    // passing a reference to a string or string literal will instantly be de referenced to a string slice on the receiving end
    either_string_or_literal(&s);
    either_string_or_literal(&string_literal);
}

fn either_string_or_literal(param: &str) {
    println!("this is a string slice: {:?}", param);
}

fn only_reference_to_array(param: &[i32; 3]) {
    println!("this is an array: {:?}", param);
}

fn only_reference_to_vector(param: &Vec<i32>) {
    println!("this is a vector: {:?}", param);
}

fn reference_to_either_array_or_vectors(param: &[i32]) {
    println!("this is a slice: {:?}", param);
}
