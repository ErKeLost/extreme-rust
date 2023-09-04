mod apple;
mod orange;
mod pear;
use rand::Rng;

fn main() {
    println!("Hello, world!");
    let gen = rand::thread_rng().gen::<i64>() % 2;
    if gen == 0 {
        apple::eat_apple();
        apple::fruit::apple::eat_apple();
    } else {
        pear::eat_pear();
    }
    orange::eat::eat_orange();
}
