// Management of sockets

// a socket is a network endpoint e.g TCP
// to use a tcp:
/*
  we bind to a port
  close the port when done
  memory and sockets should be treated alike

  Memory and Socket Problems
  Memory                Sockets
  Use after free       Use after close
  Double free          Closing twice
  Memory Leaks         Socket Leaks
  Mitigated with garbage collection  NOT mitigated with garbage collection
  Fixed with Ownership   Fixed with ownership
*/

// use std::net::TcpListener;
// use std::thread;
// use std::time::Duration;

// fn open_socket_for_five_seconds() {
//     let _listener = TcpListener::bind("127.0.0.1:5000").unwrap();
//     thread::sleep(Duration::from_secs(5));
// }

// fn main() {
//     // create socket
//     open_socket_for_five_seconds();
//     // socket will close as soon as it is back in main
//     println!("Back in main");
//     thread::sleep(Duration::from_secs(5));
// }

// Other resources managed with ownership

// Mutex<t> (Mutual Exclusion)
// - only let one thread at a time change the inner value
// - to modify the value, acquire the mutex's lock
// - Release the lock after modifying to let other threads acquire the lock
// - owner going out of scope automatically releases the lock in rust

// Rc<T> (Reference Counted)
// - Allows for mutliple owners
// - keeps track of how many owners exist
// - memory is cleaned up when the last owner goes out of scope
// - count management happens automatically when each owner goes out of scope


// Files
// - Close when done using
// - Closed automatically when owner goes out of scope

// Customizing types with the Drop trait

// The Drop Trait
// One method:drop
// drop takes &mut self
// logic neccessary to cleanup the resources any type uses

struct Noisy {
    id: i32
}

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("Noisy number {} going out of scope!", self.id);
    }
}

fn main() {
    let  _n1 = Noisy { id:1 };
    let  _n2 = Noisy { id: 2};
    println!("End of main");
}
