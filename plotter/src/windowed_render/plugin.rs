use crate::{config::AppConfig, conditional_render::setup_conditional_rendering};
use bevy::prelude::*;
use super::render_target::setup_windowed_render_target;

pub struct WindowedRenderingPlugin {
    config: AppConfig
}

impl WindowedRenderingPlugin {
    pub fn new(config: AppConfig) -> WindowedRenderingPlugin {
        WindowedRenderingPlugin {
            config
        }
    }
}

impl Plugin for WindowedRenderingPlugin {
    fn build(&self, game: &mut App) {
        let _config = &self.config;

        // Init default plugins
        game.add_plugins(DefaultPlugins);
        
        // Init rendering systems
        game
        .add_systems(Startup, setup_conditional_rendering)
        .add_systems(Startup, setup_windowed_render_target.before(setup_conditional_rendering));
    }
}