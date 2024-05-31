use crate::{
    config::AppConfig, 
    conditional_render::setup_conditional_rendering, 
    windowed_render::plugin::WindowedRenderingPlugin, 
    headless_render::plugin::HeadlessRenderingPlugin, 
    scene::setup_scene
};
use bevy::prelude::*;
use std::path::PathBuf;

pub async fn render(path: Option<PathBuf>) {
    let config = AppConfig {
        width: 1920,
        height: 1080,
        path: path
    };

    // Initiate app
    let mut game = App::new();

    // Attach app config
    game.insert_resource(config.clone());

    // Set up headless or headful rendering
    if config.path.is_some() {
        game.add_plugins(HeadlessRenderingPlugin::new(config.clone()));
    } else {
        game.add_plugins(WindowedRenderingPlugin::new(config.clone()));
    }
    
    // Set up scene
    game.add_systems(Startup, setup_scene.after(setup_conditional_rendering));

    // Run render or app window
    game.run();
}
