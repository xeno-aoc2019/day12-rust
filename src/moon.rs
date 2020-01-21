#[derive(Copy, Debug, Eq, Clone)]
pub struct Moon {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Copy, Debug, Clone)]
pub struct MoonState {
    moon: Moon,
    velocity: (i32, i32, i32),
}

pub struct MoonStates {
    moons: Vec<MoonState>,
}

impl PartialEq for Moon {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.x == other.x
    }
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
        for other in others {
            if self != other {
                let moon_adj = self.adjustment(other);
                let dx = adjustment.0 + moon_adj.0;
                let dy = adjustment.1 + moon_adj.1;
                let dz = adjustment.2 + moon_adj.2;
                adjustment = (dx, dy, dz);
            }
        }
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
    fn new(moon: &Moon) -> MoonState {
        MoonState {
            moon: moon.clone(),
            velocity: (0,0,0),
        }
    }
}

impl MoonStates {
    fn new(moons: &Vec<Moon>) -> MoonStates {
        MoonStates { moons: moons.clone().into_iter().map(|moon|MoonState::new(&moon)).collect() }
    }
}

fn axis_adjustment(value: i32, other: i32) -> i32 {
    if other < value { return -1; }
    if other > value { return 1; }
    return 0;
}