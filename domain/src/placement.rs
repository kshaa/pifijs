use glam::Vec2;

pub struct Placement {
    pub position: Vec2,
    pub scale: f32,
}

impl Placement {
    pub fn new(position: Vec2, scale: f32) -> Placement {
        Placement {
            position,
            scale,
        }
    }
}
