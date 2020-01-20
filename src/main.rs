mod input;
mod moons;

use moons::Moon;

fn main() {
    let moons: Vec<Moon> = input::get_moons("input.txt").into_iter().map(|c| Moon::new(c)).collect();
    let adjustments = moons.clone().into_iter().map(|moon| moon.adjustments(&moons));
    println!("Hello, world!");
}
