// how to create and use a mutable reference

// allows us to print out the values of the struct for inspection
#[derive(Debug)]
struct Bucket {
    liters: u32,
}

#[derive(Debug)]
struct CarPool {
    passengers: Vec<String>,
}

// in first parameters of methods
impl CarPool {
    /// Add the named passenger to the carpool
    fn pick_up(&mut self, name: String) {
        self.passengers.push(name);
    }
}

// in functions
fn pour(source: &mut Bucket, target: &mut Bucket, amount: u32) {
    source.liters -= amount;
    target.liters += amount;
}

fn main() {
    let mut bucket1 = Bucket { liters: 20 };
    let mut bucket2 = Bucket { liters: 10 };

    pour(&mut bucket1, &mut bucket2, 3);

    println!("Bucket 1: {:?}", bucket1);
    println!("Bucket 2: {:?}", bucket2);

    let mut monday_car_pool = CarPool { passengers: vec![] };

    monday_car_pool.pick_up(String::from("Ifeanyi"));
    println!("Carpool state: {:?}", monday_car_pool);

    monday_car_pool.pick_up(String::from("Dayo"));
    println!("Carpool state: {:?}", monday_car_pool);

    // Only one mutable reference is allowed at a time or multiple immutable reference to a value

    // an example of iterator invalidation bug
    //  let mut list = vec![1,2,3];
    //  for i in &list {
    //      println!("i is {}", i);
    //      list.push(i+1);
    //  }
}
// borrowing rules involving mutability
// example of a problem prevented by the borrowing rules
