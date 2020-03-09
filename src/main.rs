use std::collections::HashMap;

// what does it mean when we say variables are borrowed at the same time

// at the same time = in the same scope

fn main() {
    let mut list = vec![1, 2, 3];
    // New scope pattern
    // adding new scopes  to end all the immutable borrows before the mutable borrow

    {
        let list_first = list.first();
        let list_last = list.last();
        println!(
            "the first element is {:?} and the last is  {:?}",
            list_first, list_last
        );
    }

    *list.first_mut().expect("List was empty") += 1;

    // Temporary variable pattern
    let mut player1 = Player::new();
    let old_score = player1.score();
    player1.set_score(old_score + 1);

    // ENTRY API
    let text = "hello world hello";

    let mut freqs = HashMap::new();

    for word in text.split_whitespace() {
        // match freqs.get_mut(word) {
        //     Some(value) => *value += 1,
        //     None => {
        //         freqs.insert(word, 1);
        //     }
        // }
        *freqs.entry(word).or_insert(0) += 1;
    }

    println!("Word frequencies: {:#?}", freqs);

    // Splitting up structs
}

// split of stucts with unifying functionality together and attach them to parent structs if there are issues of mutabliliy and immutability
pub struct Stats {
    hp: u8,
    sp: u8,
}

impl Stats {
    pub fn heal(&mut self, amount: u8) {
        self.hp += amount;
        self.sp -= amount;
    }
}

pub struct Monster {
    stats: Stats,
    friends: Vec<Friend>,
}

pub struct Friend {
    loyalty: u8,
}

impl Monster {
    pub fn final_breath(&mut self) {
        if let Some(friend) = self.friends.first() {
            self.stats.heal(friend.loyalty);
            println!("Healing for  {}", friend.loyalty);
        }
    }
}

#[derive(Debug)]
struct Player {
    score: i32,
}

impl Player {
    pub fn set_score(&mut self, new_score: i32) {
        self.score = new_score;
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn new() -> Self {
        Player { score: 0 }
    }
}

// Improvements to Borrowing
// Borrows that aren't used through the end of the scope
// Borrows only used for part of an expression
// Borrows that aren't used in all arms of an if or match expression

// things that wont change
// entry api will still be used and is efficient
// won't change the need the to split structs as methods will still borrow the entire struct
// code that actually violates the borrowing rules will still be invalid
// existing code might not be updated
