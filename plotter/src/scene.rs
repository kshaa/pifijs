use crate::conditional_render::ConditionalRenderTarget;

use bevy::{
    core_pipeline::tonemapping::Tonemapping,
    prelude::*,
};

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut head_render_target: ResMut<ConditionalRenderTarget>,
) {
    // Set background color
    commands.insert_resource(ClearColor(Color::srgb_u8(0, 0, 0)));

    // Scene example for non black box picture
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::srgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        projection: Projection::Orthographic(OrthographicProjection {
            scale: 0.005,
            ..OrthographicProjection::default()
        }),
        transform: Transform::from_xyz(0.0, 9.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        tonemapping: Tonemapping::None,
        camera: Camera {
            // render to image or window
            target: head_render_target.target.take().unwrap(),
            ..default()
        },
        ..default()
    });
}
