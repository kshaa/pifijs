use crate::{
    conditional_render::ConditionalRenderTarget, 
    headless_render::{
        image_copy_driver::setup_render_target, 
        scene_controller::SceneController
    }
};

use bevy::{
    prelude::*,
    render::renderer::RenderDevice,
};

pub fn setup_headless_target(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
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
    commands.insert_resource(ConditionalRenderTarget::new(render_target))
}
