use crate::{
    conditional_render::setup_conditional_rendering, config::AppConfig, controls::update_scene_by_keyboard, headless_render::plugin::HeadlessRenderingPlugin, linestrip::Linestrip, scene::{render_scene, setup_scene}, windowed_render::plugin::WindowedRenderingPlugin
};
use bevy::prelude::*;
use std::path::PathBuf;

pub async fn render(linestrips_serialized: Option<String>, path: Option<PathBuf>) {
    // Create a parser for this and pass it through plotter params
    let linestrips: Vec<Linestrip> = (match linestrips_serialized {
        None => None,
        Some(serialized) => Linestrip::parse_strips(serialized)
    }).unwrap(); // Handle parse error
    
    // '0.,1.>0.,-1. -1.,0.>1.,0.'

    // vec!(
    //     // y axis
    //     Linestrip::new(vec!(
    //         Vec2::new(0., 1.),
    //         Vec2::new(0., -1.)
    //     )),
    //     // y axis
    //     Linestrip::new(vec!(
    //         Vec2::new(-1., 0.),
    //         Vec2::new(1., 0.)
    //     )),
    //     // Triangle A
    //     Linestrip::new(vec!(
    //         Vec2::new(-0.5, 0.5),
    //         Vec2::new(0.5, 0.5),
    //         Vec2::new(0., -0.5),
    //         Vec2::new(-0.5, 0.5)
    //     )),
    //     // Triangle B
    //     Linestrip::new(vec!(
    //         Vec2::new(0.5, 1.5),
    //         Vec2::new(1.5, 1.5),
    //         Vec2::new(1., 0.5),
    //         Vec2::new(0.5, 1.5)
    //     )),
    // );

    let hardcoded_identity_scale = 0.001;
    let hardcoded_padding = 1.20;
    let padded_identity_scale = hardcoded_identity_scale * hardcoded_padding;
    let placement = Linestrip::placement(linestrips.clone(), padded_identity_scale);

    let config = AppConfig {
        width: 1000,
        height: 1000,
        path: path,
        scale: placement.scale, // The smaller the scale, the more zoomed in we are
        position: placement.position,
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
