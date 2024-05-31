use bevy::prelude::*;
use crate::conditional_render::ConditionalRenderTarget;

// Create window rendering target
pub fn setup_windowed_render_target(mut commands: Commands) {
    commands.insert_resource(ConditionalRenderTarget::new(Default::default()));
}
