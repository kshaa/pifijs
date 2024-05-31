use crate::{
    config::AppConfig, 
    conditional_render::setup_conditional_rendering, 
    headless_render::scene_controller::SceneController
};

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use std::time::Duration;

use super::{image_copy_driver::ImageCopyPlugin, render_target::setup_headless_target, saver::CaptureFramePlugin};

pub struct HeadlessRenderingPlugin {
    config: AppConfig
}

impl HeadlessRenderingPlugin {
    pub fn new(config: AppConfig) -> HeadlessRenderingPlugin {
        HeadlessRenderingPlugin {
            config
        }
    }
}

impl Plugin for HeadlessRenderingPlugin {
    fn build(&self, game: &mut App) {
        let config = &self.config;

        game.insert_resource(SceneController::new(
            config.width,
            config.height,
        ))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                // Do not create a window on startup.
                .set(WindowPlugin {
                    primary_window: None,
                    exit_condition: bevy::window::ExitCondition::DontExit,
                    close_when_requested: false,
                }),
        );
        game
        .add_plugins(ImageCopyPlugin)
        // headless frame capture
        .add_plugins(CaptureFramePlugin)
        .add_plugins(ScheduleRunnerPlugin::run_loop(
            // Run 60 times per second.
            Duration::from_secs_f64(1.0 / 60.0),
        ))
        .init_resource::<SceneController>()
        .add_systems(Startup, setup_conditional_rendering)
        .add_systems(Startup, setup_headless_target.before(setup_conditional_rendering));
    }
}