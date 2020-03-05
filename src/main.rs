// enums are for choice between a set of values
// structs are the same attributes for a data type

// struct definition
struct HockeyPlayer {
    name: String,
    number: u8,
    goals_ytd: u8,
}

// tuple structs
struct Triangle(u32, u32, u32);

fn is_equilateral(triangle: Triangle) -> bool {
    triangle.0 == triangle.1 && triangle.1 == triangle.2
}

// you can use tuples to define new data types out of primitives
// here we can wrap a u8 for our own use as meters
// it is called new type pattern

struct Meters(u8);

// Unit structs have no initial value
// struct MyStruct;
// let s = MyStruct;

fn add_distances(d1: Meters, d2: Meters) -> Meters {
    Meters(d1.0 + d2.0)
}

fn main() {
    let mut player = HockeyPlayer {
        name: String::from("Bryan Rust"),
        number: 17,
        goals_ytd: 7,
    };

    player.goals_ytd += 1;

    println!(
        "{} of number {} has scored {} goals this season",
        player.name, player.number, player.goals_ytd
    );

    let triangle = Triangle(3, 4, 5);
    println!("{}", is_equilateral(triangle));

    let distance1 = Meters(3);
    let distance2 = Meters(7);

    let distance3 = add_distances(distance1, distance2);
    println!("{}", distance3.0);
}

// blending enums with structs
//  enum Clock {
//      Sundial { hours: u8 },
//      Digital {hours: u8, minutes: u8 },
//  }

//  fn main() {
//      let clock = Clock::Digital {
//          hours: 9,
//          minutes: 10,
//      };
//  }
