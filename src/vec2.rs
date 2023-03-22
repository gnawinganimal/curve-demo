use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone, Copy, PartialEq, PartialOrd, Default, Debug)]
pub struct Vec2(pub f32, pub f32);

pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2(x, y)
}

impl Add<f32> for Vec2 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self {
        Self(self.0 + rhs, self.1 + rhs)
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<f32> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self {
        Self(self.0 - rhs, self.1 - rhs)
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Vec2) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Vec2) -> Self {
        Self(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl Div<Vec2> for Vec2 {
    type Output = Self;

    fn div(self, rhs: Vec2) -> Self {
        Self(self.0 / rhs.0, self.1 / rhs.1)
    }
}
