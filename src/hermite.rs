use crate::{Vec2, Curve};

#[derive(Copy, Clone, Default, Debug)]
pub struct Hermite {
    p0: Vec2,
    v0: Vec2,
    p1: Vec2,
    v1: Vec2,
}

impl Hermite {
    pub fn new(p0: Vec2, v0: Vec2, p1: Vec2, v1: Vec2) -> Self {
        Self {
            p0,
            v0,
            p1,
            v1,
        }
    }
}

pub fn herm(p0: Vec2, v0: Vec2, p1: Vec2, v1: Vec2) -> Hermite {
    Hermite::new(p0, v0, p1, v1)
}

impl Curve for Hermite {
    fn first(&self) -> Vec2 {
        self.p0
    }

    fn last(&self) -> Vec2 {
        self.p1
    }

    fn join_c0(&self, other: &Self) -> bool {
        self.p1 == other.p0
    }

    fn join_c1(&self, other: &Self) -> bool {
        self.v1 == other.v0
    }

    fn get(&self, t: f32) -> Option<Vec2> {
        if t < 0. || t > 1. {
            return None;
        }

        let t3 = self.p0 * (2. * t.powf(3.) - 3. * t.powf(2.) + 1.);
        let t2 = self.v0 * (t.powf(3.) - 2. * t.powf(2.) + t);
        let t1 = self.p1 * (-2. * t.powf(3.) + 3. * t.powf(2.));
        let t0 = self.v1 * (t.powf(3.) - t.powf(2.));

        Some(t3 + t2 + t1 + t0)
    }
}
