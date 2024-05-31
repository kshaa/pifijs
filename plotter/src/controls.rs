use bevy::prelude::*;
use bevy::{ecs::query::With, gizmos::config::{DefaultGizmoConfigGroup, GizmoConfigStore}, input::{keyboard::KeyCode, ButtonInput}, render::camera::OrthographicProjection, time::Time, transform::components::Transform};
use crate::config::AppConfig;

pub fn update_scene_by_keyboard(
    mut app_config: ResMut<AppConfig>,
    mut config_store: ResMut<GizmoConfigStore>,
    mut projections: Query<&mut OrthographicProjection>,
    mut camera_transforms: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();

    // Line width
    let is_line_width = keyboard.pressed(KeyCode::ControlLeft);
    if is_line_width {
        if keyboard.pressed(KeyCode::ArrowRight) {
            config.line_width += 10. * time.delta_seconds();
            config.line_width = config.line_width.clamp(0., 50.);
        }
        if keyboard.pressed(KeyCode::ArrowLeft) {
            config.line_width -= 10. * time.delta_seconds();
            config.line_width = config.line_width.clamp(0., 50.);
        }
    }

    // Zoom
    let is_zoom = keyboard.pressed(KeyCode::ShiftLeft);
    if keyboard.pressed(KeyCode::ShiftLeft) {
        let multiplier = 1. + (1. * time.delta_seconds());
        info!("Old value {}", app_config.scale);
        if keyboard.pressed(KeyCode::ArrowRight) {
            for mut projection in &mut projections {
                let mut new_value = app_config.scale;
                new_value /= multiplier;
                new_value = new_value.clamp(0.0, 50.);
                projection.scale = new_value;
                app_config.scale = new_value;
            }
        }
        if keyboard.pressed(KeyCode::ArrowLeft) {
            for mut projection in &mut projections {
                let mut new_value = app_config.scale;
                new_value *= multiplier;
                new_value = new_value.clamp(0.0, 50.);
                projection.scale = new_value;
                app_config.scale = new_value;
            }
        }    
    }

    // Position
    if !is_line_width && !is_zoom {
        let screen_movement = 250.0;
        let absolute_movement = screen_movement * app_config.scale * time.delta_seconds();

        if keyboard.pressed(KeyCode::ArrowLeft) {
            for mut camera_transform in &mut camera_transforms {
                camera_transform.translation.x -= absolute_movement;
            }
        }
        if keyboard.pressed(KeyCode::ArrowRight) {
            for mut camera_transform in &mut camera_transforms {
                camera_transform.translation.x += absolute_movement;
            }
        }
        if keyboard.pressed(KeyCode::ArrowUp) {
            for mut camera_transform in &mut camera_transforms {
                camera_transform.translation.y += absolute_movement;
            }
        }
        if keyboard.pressed(KeyCode::ArrowDown) {
            for mut camera_transform in &mut camera_transforms {
                camera_transform.translation.y -= absolute_movement;
            }
        }
    }
}
