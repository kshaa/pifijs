use std::{cmp::Ordering, convert::identity};
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

    pub fn parse_coord(serialized: String) -> Option<f32> {
        serialized.parse::<f32>().ok()
    }

    pub fn parse_coords(serialized: String) -> Option<Vec2> {
        let coords: Vec<String> = serialized.split(",").map(|c| { c.to_string() } ).collect();
        let serialized_x = coords.get(0);
        let serialized_y = coords.get(1);
        let parsed_coords = match (serialized_x, serialized_y) {
            (Some(x), Some(y)) => (Linestrip::parse_coord(x.clone()), Linestrip::parse_coord(y.clone())),
            _ => (None, None),
        };

        match parsed_coords {
            (Some(x), Some(y)) => Some(Vec2::new(x, y)),
            _ => None
        }
    }

    pub fn parse_strip(serialized: String) -> Option<Linestrip> {
        let parts: Vec<&str> = serialized.split(">").collect();
        let parsed_parts = parts.iter().map(|p| { Linestrip::parse_coords(p.to_string()) });

        let successfully_parsed_parts: Vec<Vec2> = parsed_parts.filter_map(identity).collect();

        if successfully_parsed_parts.len() == parts.len() {
            Some(Linestrip::new(successfully_parsed_parts))
        } else {
            None
        }
    }

    pub fn parse_strips(serialized: String) -> Option<Vec<Linestrip>> {
        let parts: Vec<&str> = serialized.split_whitespace().collect();
        let parsed_parts = parts.iter().map(|p| { Linestrip::parse_strip(p.to_string()) });

        let successfully_parsed_parts: Vec<Linestrip> = parsed_parts.filter_map(identity).collect();

        if successfully_parsed_parts.len() == parts.len() {
            Some(successfully_parsed_parts)
        } else {
            None
        }
    }
}
