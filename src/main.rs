mod input;
mod moon;

use moon::Moon;

fn main() {
    let moons: Vec<Moon> = input::get_moons("input.txt").into_iter().map(|c| Moon::new(c)).collect();
    let adjustments = moons.clone().into_iter().map(|moon| moon.adjustments(&moons));
    let new_moons : Vec<Moon> = moons.clone().into_iter().map(|moon| moon.adjust(&(1,1,1))).collect();
    for new_moon in new_moons {
        println!("{}", new_moon);
    }
    println!("Hello, world!");
}
