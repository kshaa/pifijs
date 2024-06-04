use glam::Vec2;
use approx::{RelativeEq, AbsDiffEq};
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct WrappedVec2(pub Vec2);

impl Deref for WrappedVec2 {
    type Target = Vec2;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Eq for WrappedVec2 {}

impl PartialEq for WrappedVec2 {
    fn eq(&self, other: &Self) -> bool {
        let epsilon = f32::default_max_relative();
        let x_eq = self.deref().x.abs_diff_eq(&other.deref().x, epsilon);
        let y_eq = self.deref().y.abs_diff_eq(&other.deref().y, epsilon);
        x_eq && y_eq
    }
}
