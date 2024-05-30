use std::path::PathBuf;
use bevy::ecs::system::Resource;

// Parameters of resulting image
#[derive(Resource, Clone)]
pub struct AppConfig {
    pub width: u32,
    pub height: u32,
    pub path: PathBuf,
    pub single_image: bool,
}
