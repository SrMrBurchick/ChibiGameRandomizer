use json::{object, JsonValue};
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng(); // Thread-local random number generator
    // Generate a random integer in a range
    let random_int = rng.gen_range(1..101); // Generates a number between 1 and 100

    // Random boolean
    let random_bool: bool = rng.gen();

    let output = object! {
        "RandomNumber": random_int,
        "RandomBool": random_bool
    };

    println!("{}", output.dump());
}
