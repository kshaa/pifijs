use std::path::PathBuf;
use bevy::{ecs::system::Resource, math::Vec2};

use crate::linestrip::Linestrip;

// Parameters of resulting image
#[derive(Resource, Clone)]
pub struct AppConfig {
    pub width: u32,
    pub height: u32,
    pub path: Option<PathBuf>,
    pub scale: f32,
    pub position: Vec2,
    pub linestrips: Vec<Linestrip>,
}
