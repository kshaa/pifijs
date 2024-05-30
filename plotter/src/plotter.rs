use crate::{
    config::AppConfig, 
    headless_render::{image_copy_driver::ImageCopyPlugin, saver::CaptureFramePlugin, scene_controller::SceneController}, 
    scene::setup_scene
};
use bevy::{
    app::ScheduleRunnerPlugin,
    prelude::*,
};
use std::{path::PathBuf, time::Duration};

pub async fn render(path: &PathBuf) {
    let config = AppConfig {
        width: 1920,
        height: 1080,
        path: path.clone(),
        single_image: true,
    };

    // Initiate app
    let mut game = App::new();

    // Attach app config
    game.insert_resource(config.clone());

    // Set up headless rendering
    game.insert_resource(SceneController::new(
            config.width,
            config.height,
            config.single_image,
        ))
        // .insert_resource(render_lock.sender)
        .insert_resource(ClearColor(Color::srgb_u8(0, 0, 0)))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                // Do not create a window on startup.
                .set(WindowPlugin {
                    primary_window: None,
                    exit_condition: bevy::window::ExitCondition::DontExit,
                    close_when_requested: false,
                }),
        )
        .add_plugins(ImageCopyPlugin)
        // headless frame capture
        .add_plugins(CaptureFramePlugin)
        .add_plugins(ScheduleRunnerPlugin::run_loop(
            // Run 60 times per second.
            Duration::from_secs_f64(1.0 / 60.0),
        ))
        .init_resource::<SceneController>();
    
    // Set up scene
    game.add_systems(Startup, setup_scene);

    // Run render
    game.run();
    info!("Render finished!");
}
