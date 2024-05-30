//! This example illustrates how to make headless renderer
//! derived from: <https://sotrh.github.io/learn-wgpu/showcase/windowless/#a-triangle-without-a-window>
//! It follows this steps:
//! 1. Render from camera to gpu-image render target
//! 2. Copy from gpu image to buffer using `ImageCopyDriver` node in `RenderGraph`
//! 3. Copy from buffer to channel using `receive_image_from_buffer` after `RenderSet::Render`
//! 4. Save from channel to random named file using `scene::update` at `PostUpdate` in `MainWorld`
//! 5. Exit if `single_image` setting is set

/**
 * This whole folder is copy-pasted and split-up from Bevy examples
 * Source: https://github.com/bevyengine/bevy/blob/e208fb70f5b49518fe196d02703ab97060bf6378/examples/app/headless_renderer.rs
 */
pub mod scene_controller;
pub mod gpu_crossbeam;
pub mod saver;
pub mod image_copy_driver;
