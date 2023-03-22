use crate::{Vec2, Curve};

#[derive(Default)]
pub struct Spline<C: Curve + Clone> {
    curves: Vec<C>,
}

impl<C: Curve + Clone> Spline<C> {
    pub fn new() -> Self {
        Self {
            curves: Vec::new(),
        }
    }

    pub fn len(&self) -> f32 {
        self.curves.len() as f32
    }

    pub fn first(&self) -> Option<Vec2> {
        Some(self.curves.first()?.first())
    }

    pub fn last(&self) -> Option<Vec2> {
        Some(self.curves.last()?.last())
    }

    pub fn get(&self, u: f32) -> Option<Vec2> {
        if u == self.len() {
            return self.last()
        }

        let i = u.floor() as usize;
        let t = u - u.floor();

        self.curves.get(i)?.get(t)
    }
}

impl<C: Curve + Clone> From<&[C]> for Spline<C> {
    fn from(slice: &[C]) -> Self {
        Self {
            curves: Vec::from(slice),
        }
    }
}
