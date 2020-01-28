use std::fmt;
use std::fmt::Formatter;

#[derive(Copy, Debug, Eq, Clone, Hash)]
pub struct Moon {
    x: i32,
    y: i32,
    z: i32,
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Moon(x={} y={} z={})", self.x, self.y, self.z)
    }
}

#[derive(Copy, Debug, Clone, Eq, Hash)]
pub struct MoonState {
    moon: Moon,
    velocity: (i32, i32, i32),
}

impl fmt::Display for MoonState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "MoonState({}, velocity=({},{},{}))", self.moon, self.velocity.0, self.velocity.1, self.velocity.2)
    }
}

impl PartialEq for MoonState {
    fn eq(&self, other: &Self) -> bool {
        let e1 = self.moon == other.moon;
        let e2 = self.velocity == other.velocity;
        e1 && e2
    }
}


#[derive(Debug, Clone, Eq, Hash)]
pub struct MoonStates {
    step: u32,
    states: Vec<MoonState>,
}

impl PartialEq for Moon {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.x == other.x
    }
}

fn add3(a: &(i32, i32, i32), b: &(i32, i32, i32)) -> (i32, i32, i32) {
    let x = a.0.clone() + b.0.clone();
    let y = a.1.clone() + b.1.clone();
    let z = a.2.clone() + b.2.clone();
    (x, y, z)
}

impl Moon {
    pub fn adjust(&self, adjustment: &(i32, i32, i32)) -> Moon {
        let x = self.x + adjustment.0;
        let y = self.y + adjustment.1;
        let z = self.z + adjustment.2;
        Moon { x, y, z }
    }

    pub fn adjustments(&self, others: &Vec<Moon>) -> (i32, i32, i32) {
        let mut adjustment = (0, 0, 0);
        println!("Adjustment for {}", &self);
        for other in others {
//            if self != other {
            let moon_adj = self.adjustment(other);
            println!("  {} :: {} : ({},{},{})", &self, &other, &moon_adj.0, &moon_adj.1, &moon_adj.2);
            // adjustment = (dx, dy, dz);
            adjustment = add3(&adjustment, &moon_adj);
//            }
        }
        println!("{} : adjustment: {:?}", &self, &adjustment);
        adjustment
    }


    fn adjustment(&self, other: &Moon) -> (i32, i32, i32) {
        let x = axis_adjustment(self.x, other.x);
        let y = axis_adjustment(self.y, other.y);
        let z = axis_adjustment(self.z, other.z);
        (x, y, z)
    }

    pub fn new(coords: (i32, i32, i32)) -> Moon {
        Moon { x: coords.0, y: coords.1, z: coords.2 }
    }
}

impl MoonState {
    pub fn new(moon: &Moon) -> MoonState {
        MoonState {
            moon: moon.clone(),
            velocity: (0, 0, 0),
        }
    }
    pub fn newWithVelo(moon: &Moon, velo: &(i32, i32, i32)) -> MoonState {
        MoonState {
            moon: moon.clone(),
            velocity: velo.clone(),
        }
    }
    pub fn energy(&self) -> i32 {
        let potential = self.moon.x.abs() + self.moon.y.abs() + self.moon.z.abs();
        let kinetic = self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs();
        potential * kinetic
    }
}

impl fmt::Display for MoonStates {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "Moonstates {\n");
        write!(f, "  step: {}\n", &self.step);
        for s in self.states.clone() {
            write!(f, "  {}\n", s);
        }
        write!(f, "{}", "}")
    }
}

impl PartialEq for MoonStates {
    fn eq(&self, other: &Self) -> bool {
        let e1 = self.states[0] == other.states[0];
        let e2 = self.states[1] == other.states[1];
        let e3 = self.states[2] == other.states[2];
        let e4 = self.states[3] == other.states[3];
        e1 && e2 && e3 && e4
    }
}


impl MoonStates {
    pub fn new(moons: &Vec<Moon>) -> MoonStates {
        MoonStates { step: 0, states: moons.clone().into_iter().map(|moon| MoonState::new(&moon)).collect() }
    }
    pub fn moons(&self) -> Vec<Moon> {
        self.states.clone().into_iter().map(|state| state.moon).collect()
    }

    pub fn nextStates(&self) -> MoonStates {
        let mut result: Vec<MoonState> = vec!();
        let moons = self.moons();
        for state in self.states.clone() {
            let adjustment = state.moon.adjustments(&moons);
            let veloX = state.velocity.0 + adjustment.0;
            let veloY = state.velocity.1 + adjustment.1;
            let veloZ = state.velocity.2 + adjustment.2;
            let newVelocity = (veloX, veloY, veloZ);
            let newMoon = state.moon.adjust(&newVelocity);
            let newState = MoonState { moon: newMoon, velocity: newVelocity };
            result.push(newState);
        }

        MoonStates { step: self.step + 1, states: result }
    }

    pub fn energy(&self) -> i32 {
        let mut e = 0;
        for ms in self.states.clone() {
            e = e + ms.energy();
//            println!("Adding moon energy: {} ", e);
        }
        e
    }
}

fn axis_adjustment(value: i32, other: i32) -> i32 {
    let result = match value {
        x if x > other => -1,
        x if x < other => 1,
        default => 0
    };
//    println!("axis_adjustment: value={}, other={} => {}", value, other, result);
    result
}