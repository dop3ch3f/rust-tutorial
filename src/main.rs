// Panic Concept
// panic! macro
// #[derive(Debug)]
// enum Platform {
//     Windows,
//     Linux,
//     Macos,
// }

// impl Platform {
//     fn parse(platform_arg: &str) -> Platform {
//         if platform_arg == "windows" {
//             Platform::Windows
//         } else if platform_arg == "linux" {
//             Platform::Linux
//         } else if platform_arg == "macos" {
//             Platform::Macos
//         } else {
//             panic!(
//                 "Unknown platform: {}. Valid values: windows, linux, macos",
//                 platform_arg
//             );
//         }
//     }
// }

// fn main() {
//     let platform_arg = "win";
//     let platform = Platform::parse(platform_arg);
//     println!("Producing output for {:?}", platform);
// }

// Situations in which to panic

// when continuing will be incorrect
// no way for calling code to recover
// when problem must be fixed in code not by user input
// Panic first, change later

// Other macros that call panic!

// unreachable! - impossible to get to this spot

enum DoorState {
    Opened,
    Closed,
}

enum DoorAction {
    Open,
    Close,
}

fn take_action(current_state: DoorState, action: DoorAction) {
    match (current_state, action) {
        (DoorState::Opened, DoorAction::Close) => unimplemented!(),
        (DoorState::Closed, DoorAction::Open) => {
            // code to open the door goes here
        }
        // if you get here, a programming mistake has been made
        _ => unreachable!(),
    }
}

fn main() {
    take_action(DoorState::Opened, DoorAction::Open);
}
