use crate::{conditional_render::ConditionalRenderTarget, config::AppConfig};

use bevy::{
    core_pipeline::tonemapping::Tonemapping,
    prelude::*,
};

pub fn setup_scene(
    mut commands: Commands,
    mut head_render_target: ResMut<ConditionalRenderTarget>,
    config: Res<AppConfig>,
) {
    // Set background color
    commands.insert_resource(ClearColor(Color::WHITE));

    // Configure camera
    // commands.spawn(Camera2dBundle::default());

    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: config.scale,
            ..OrthographicProjection::default()
        },
        transform: Transform::from_xyz(config.position.x, config.position.y, 10.0),
        tonemapping: Tonemapping::None,
        camera: Camera {
            // render to image or window
            target: head_render_target.target.take().unwrap(),
            ..default()
        },
        ..default()
    });
}

pub fn render_scene(
    app_config: Res<AppConfig>,
    mut gizmos: Gizmos,
) {
    for linestrip in app_config.linestrips.clone() {
        gizmos.linestrip_2d(linestrip.deref(), Color::BLACK);    
    }
}
