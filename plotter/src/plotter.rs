use crate::{
    conditional_render::setup_conditional_rendering, config::AppConfig, controls::update_scene_by_keyboard, headless_render::plugin::HeadlessRenderingPlugin, linestrip::Linestrip, scene::{render_scene, setup_scene}, windowed_render::plugin::WindowedRenderingPlugin
};
use bevy::prelude::*;
use std::path::PathBuf;

pub async fn render(path: Option<PathBuf>) {
    // Create a parser for this and pass it through plotter params
    let linestrips: Vec<Linestrip> = vec!(
        // y axis
        Linestrip::new(vec!(
            Vec2::new(0., 1.),
            Vec2::new(0., -1.)
        )),
        // y axis
        Linestrip::new(vec!(
            Vec2::new(-1., 0.),
            Vec2::new(1., 0.)
        )),
        // Triangle
        Linestrip::new(vec!(
            Vec2::new(-0.5, 0.5),
            Vec2::new(0.5, 0.5),
            Vec2::new(0., -0.5),
            Vec2::new(-0.5, 0.5)
        )),
    );

    let config = AppConfig {
        width: 1000,
        height: 1000,
        path: path,
        scale: 0.002, // The smaller the scale, the more zoomed in we are
        position: Vec2::ZERO,
        linestrips,
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
    
    // Initial scene (headless)
    game.add_systems(Startup, setup_scene.after(setup_conditional_rendering));
    game.add_systems(Startup, render_scene.after(setup_conditional_rendering));
    
    // Continuous scene (windowed)
    game.add_systems(Update, update_scene_by_keyboard);
    game.add_systems(Update, render_scene.after(update_scene_by_keyboard));

    // Run render or app window
    game.run();
}
