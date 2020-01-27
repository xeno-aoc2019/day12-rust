mod input;
mod moon;

use moon::{Moon, MoonState, MoonStates};

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
    processInput(moons);
//    processTest(&testMoons);
}

fn processTest(moons: &Vec<Moon>) {
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

fn processInput(moons: Vec<Moon>) {
    let moonStates = MoonStates::new(&moons);
    println!("Initial: {}", moonStates);
    let step100 = processTimes(&moonStates, 1000);
    println!("step100:");
    println!("{}", &step100);
    println!("Energy: {}", &step100.energy())
}
