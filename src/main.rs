// Why generic lifetime paramenters are needed
// - Specify how lifetimes of references are related
// - So the compiler can ensure te validity of references

// Syntax of generic lifetime parameters

// extern crate rand;

// fn simulate_game<'a>(home: &'a str, away: &'a str) -> &'a str {
//     if rand::random() {
//         home
//     } else {
//         away
//     }
// }

// fn main() {
//     print!("{}",simulate_game("home","away"));
// }

// Why Lifetime parameters are necessary

// - Express your intent in the signature
// - Keeps compiler analysis local
// - Compiler can't tell what lifetimes should be in complex cases

// Compare and contrast generic type and lifetime parameters

    // Generic Types vs Generic Lifetimes
    // - Both are declared in angle brackets(ex: <'a, T>)
    // - Both are used in structs, enums, functions, methodsm traits etc.
    // - Generic over types                  - Generic over scopes
    // - Names in UpperCamelCase             - Names in snake_case
    // - Generates code for each usage       - Used during analysis only

// Adding lifetime parameters to an example

// pub struct Stemmer {
//     pub suffix: String,
// }

// impl Stemmer {
//     pub fn stem<'a>(&self, word: &'a str) -> &'a str {
//         if  word.ends_with(&self.suffix) {
//             let index = word.rfind(&self.suffix).expect("Should be found because ends_with returned true");
//             &word[0..index]
//         } else {
//             word
//         }
//     }
// }

// fn main() {
//    let word = String::from("credited");
//    let word_stem = {
//        let stemmer = Stemmer {
//            suffix: String::from("ed"),
//        };
//        stemmer.stem(&word)
//    };
//    println!("the stem of {} is  {}", word, word_stem);
// }



// Strategy for fixing lifetime parameter errors

// - Read entire error messages and help text
// - analyse generic lifetimes and specify how you intend them to be related
// - analyse concrete lifetimes where using definitions