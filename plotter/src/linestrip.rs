use std::cmp::Ordering;

use bevy::math::Vec2;

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

#[derive(Clone)]
pub struct Linestrip {
    pub points: Vec<Vec2>
}
impl Linestrip {
    pub fn new(points: Vec<Vec2>) -> Linestrip {
        Linestrip { points }
    }

    pub fn placement(linestrips: Vec<Linestrip>, identity_scale: f32) -> Placement {
        fn cmp_float(a: &f32, b: &f32) -> Ordering {
            a.partial_cmp(b).unwrap_or(Ordering::Equal)
        }

        let xs = linestrips.iter().flat_map(|s| { s.points.iter().map(|p| { p.x }) });
        let ys = linestrips.iter().flat_map(|s| { s.points.iter().map(|p| { p.y }) });
        let min_x = xs.clone().min_by(cmp_float).unwrap();
        let max_x = xs.max_by(cmp_float).unwrap();
        let min_y = ys.clone().min_by(cmp_float).unwrap();
        let max_y = ys.max_by(cmp_float).unwrap();

        let position = Vec2 {
            x: (min_x + max_x) / 2.,
            y: (min_y + max_y) / 2.,
        };

        let width = max_x - min_x;
        let height = max_y - min_y;
        let max_size = width.max(height);
        let scale = identity_scale * max_size;

        Placement::new(position, scale)
    }
}
