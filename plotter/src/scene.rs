use crate::{
    headless_render::image_copy_driver::setup_render_target, 
    headless_render::scene_controller::SceneController
};

use bevy::{
    core_pipeline::tonemapping::Tonemapping,
    prelude::*,
    render::renderer::RenderDevice,
};

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut scene_controller: ResMut<SceneController>,
    render_device: Res<RenderDevice>,
) {
    let render_target = setup_render_target(
        &mut commands,
        &mut images,
        &render_device,
        &mut scene_controller,
        // pre_roll_frames should be big enough for full scene render,
        // but the bigger it is, the longer example will run.
        // To visualize stages of scene rendering change this param to 0
        // and change AppConfig::single_image to false in main
        // Stages are:
        // 1. Transparent image
        // 2. Few black box images
        // 3. Fully rendered scene images
        // Exact number depends on device speed, device load and scene size
        40,
        "main_scene".into(),
    );

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
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        tonemapping: Tonemapping::None,
        camera: Camera {
            // render to image
            target: render_target,
            ..default()
        },
        ..default()
    });
}
