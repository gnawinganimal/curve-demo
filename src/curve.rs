use crate::Vec2;

pub trait Curve {
    fn get(&self, t: f32) -> Option<Vec2>;

    fn first(&self) -> Vec2;

    fn last(&self) -> Vec2;

    fn join_c0(&self, other: &Self) -> bool;

    fn join_c1(&self, other: &Self) -> bool;
}
