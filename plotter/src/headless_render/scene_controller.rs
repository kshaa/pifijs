use bevy::prelude::*;

/// Capture image state
#[derive(Debug, Default)]
pub enum SceneState {
    #[default]
    // State before any rendering
    BuildScene,
    // Rendering state, stores the number of frames remaining before saving the image
    Render(u32),
}

/// Capture image settings and state
#[derive(Debug, Default, Resource)]
pub struct SceneController {
    pub state: SceneState,
    pub name: String,
    pub width: u32,
    pub height: u32,
}

impl SceneController {
    pub fn new(width: u32, height: u32) -> SceneController {
        SceneController {
            state: SceneState::BuildScene,
            name: String::from(""),
            width,
            height,
        }
    }
}