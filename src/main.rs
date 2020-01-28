mod input;
mod moon;

use moon::{Moon, MoonState, MoonStates};
use std::collections::HashSet;

fn processTimes(states: &MoonStates, count: usize) -> moon::MoonStates {
    if count > 0 {
        processTimes(&states.nextStates(), count - 1)
    } else {
        states.clone()
    }
}


fn main() {
    let moons = getTaskInput();
    let testMoons = makeTestInput();
//    process_1000(moons);
    find_loop(moons);
//    processTest(&testMoons);
    println!("Should be true:  {}", ((1, 2, 3) == (1, 2, 3)));
    println!("Should be false: {}", ((1, 2, 2) == (1, 2, 3)));
}

fn process_test(moons: &Vec<Moon>) {
    let t0 = MoonStates::new(&moons);
    let t1 = t0.nextStates();
    let t2 = t1.nextStates();
    println!("T0 : {}", t0);
    println!("T1 : {}", t1);
    println!("T2 : {}", t2);
    let t3 = t2.nextStates();
    println!("T3 : {}", t3);
    let t10 = processTimes(&t0, 10);
    println!("T10: {} energy={}", t10, t10.energy());
}

fn makeTestInput() -> Vec<Moon> {
    let mut moons = vec!();
    moons.push(Moon::new((-1, 0, 2)));
    moons.push(Moon::new((2, -10, -7)));
    moons.push(Moon::new((4, -8, 8)));
    moons.push(Moon::new((3, 5, -1)));
    moons
}

fn getTaskInput() -> Vec<Moon> {
    let moons: Vec<Moon> = input::get_moons("input.txt").into_iter().map(|c| Moon::new(c)).collect();
    moons
}

fn find_loop(moons: Vec<Moon>) {
    let mut moonStates = MoonStates::new(&moons);
    let mut prevs: HashSet<MoonStates> = HashSet::new();
    while !prevs.contains(&moonStates) {
        prevs.insert(moonStates.clone());
        moonStates = moonStates.nextStates();
    }
    let loopStart = prevs.get(&moonStates);
    println!("Loop start: {}", loopStart.unwrap());
    println!("Loop end:   {}", moonStates);
}

fn process_1000(moons: Vec<Moon>) {
    let moonStates = MoonStates::new(&moons);
    println!("Initial: {}", moonStates);
    let step100 = processTimes(&moonStates, 1000);
    println!("step100:");
    println!("{}", &step100);
    println!("Energy: {}", &step100.energy())
}
