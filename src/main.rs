use pizza::*;
use std::time::{Duration, Instant};

/// Pizza Parlor
/// Example code of using the `pizza` library.
/// Feel free to modify this code to test your implementation.
fn main() {
    let faucet = Faucet::new();
    let sack = Sack::new(30);
    let jar = Jar::new(40);
    let shaker = Shaker::new(50);

    let breadboard = BreadBoard::new(faucet, sack, shaker, jar);

    let grater = Grater::new(5);
    let can = Can::new(20);
    let prep_table = PrepTable::new(breadboard, can, grater, 4);

    let oven = Oven::new(prep_table, Duration::from_millis(100));

    for pizza in oven {
        println!("{:?} got: {:?}", Instant::now(), pizza);
    }
}
