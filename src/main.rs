struct HockeyPlayer {
    name: String,
    position: String,
}

// methods
impl HockeyPlayer {
    // to just read use &self and to write &mut self
    fn shoot_puck(&self, seconds_remaining: u16) -> String {
        if seconds_remaining < 300 {
            let result = match self.position.as_str() {
                "Wing" => "Goal",
                _ => "Miss",
            };
            return String::from(result);
        } else {
            return String::from("Goal");
        }
    }
}

// associated function
impl HockeyPlayer {
    fn new(name: String, position: String) -> HockeyPlayer {
        HockeyPlayer {
            name: name,
            position: position,
        }
    }
}

fn main() {
    let player = HockeyPlayer::new(String::from("Bryan Rust"), String::from("Wing"));
    let value = player.shoot_puck(10000);
    println!("{} just had a {}", player.name, value);
}
