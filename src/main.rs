// Properties of enums
// can only be one value at a time
// can list (enumerate) all possible values
// do not change on run time

// defining enum variants that hold data
enum Clock {
    Sundial(u8),
    Digital(u8, u8),
    Analog(u8, u8, u8),
}

fn tell_time(clock: Clock) {
    match clock {
        Clock::Sundial(hours) => println!("{} o'clock", hours),
        Clock::Analog(hours, minutes, seconds) => println!("{}:{}:{}", hours, minutes, seconds),
        Clock::Digital(hours, minutes) => println!("{}:{}", hours, minutes),
    }
}

fn main() {
    // using an enum
    tell_time(Clock::Analog(9, 25, 30));
    tell_time(Clock::Digital(8, 25));
    tell_time(Clock::Sundial(10));
}
