use bevy::math::Vec2;

#[derive(Clone)]
pub struct Linestrip {
    pub points: Vec<Vec2>
}
impl Linestrip {
    pub fn new(points: Vec<Vec2>) -> Linestrip {
        Linestrip { points }
    }
}
