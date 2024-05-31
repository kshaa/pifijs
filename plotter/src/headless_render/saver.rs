

use crate::{
    config::AppConfig, 
    headless_render::{
        gpu_crossbeam::MainWorldReceiver, 
        scene_controller::{SceneController, SceneState}
    }
};
use bevy::{
    app::AppExit,
    prelude::*,
    render::{
        renderer::RenderDevice,
        texture::TextureFormatPixelInfo,
    },
};

/// Setups image saver
pub struct CaptureFramePlugin;
impl Plugin for CaptureFramePlugin {
    fn build(&self, app: &mut App) {
        info!("Adding CaptureFramePlugin");
        app.add_systems(PostUpdate, update);
    }
}

/// CPU-side image for saving
#[derive(Component, Deref, DerefMut)]
pub struct ImageToSave(pub Handle<Image>);

// Takes from channel image content sent from render world and saves it to disk
pub fn update(
    config: Res<AppConfig>,
    images_to_save: Query<&ImageToSave>,
    receiver: Res<MainWorldReceiver>,
    // mut sender: ResMut<RenderLockSender>,
    mut images: ResMut<Assets<Image>>,
    mut scene_controller: ResMut<SceneController>,
    mut app_exit_writer: EventWriter<AppExit>,
) {
    if let SceneState::Render(n) = scene_controller.state {
        if n < 1 {
            // We don't want to block the main world on this,
            // so we use try_recv which attempts to receive without blocking
            let mut image_data = Vec::new();
            while let Ok(data) = receiver.try_recv() {
                // image generation could be faster than saving to fs,
                // that's why use only last of them
                image_data = data;
            }
            if !image_data.is_empty() {
                for image in images_to_save.iter() {
                    // Fill correct data from channel to image
                    let img_bytes = images.get_mut(image.id()).unwrap();

                    // We need to ensure that this works regardless of the image dimensions
                    // If the image became wider when copying from the texture to the buffer,
                    // then the data is reduced to its original size when copying from the buffer to the image.
                    let row_bytes = img_bytes.width() as usize
                        * img_bytes.texture_descriptor.format.pixel_size();
                    let aligned_row_bytes = RenderDevice::align_copy_bytes_per_row(row_bytes);
                    if row_bytes == aligned_row_bytes {
                        img_bytes.data.clone_from(&image_data);
                    } else {
                        // shrink data to original image size
                        img_bytes.data = image_data
                            .chunks(aligned_row_bytes)
                            .take(img_bytes.height() as usize)
                            .flat_map(|row| &row[..row_bytes.min(row.len())])
                            .cloned()
                            .collect();
                    }

                    // Create RGBA Image Buffer
                    let img = match img_bytes.clone().try_into_dynamic() {
                        Ok(img) => img.to_rgba8(),
                        Err(e) => panic!("Failed to create image buffer {e:?}"),
                    };

                    // Prepare directory for images, test_images in bevy folder is used here for example
                    // You should choose the path depending on your needs
                    let image_path = config.path.clone().unwrap();
                    info!("Saving image {image_path:?}");

                    // Finally saving image to file, this heavy blocking operation is kept here
                    // for example simplicity, but in real app you should move it to a separate task
                    if let Err(e) = img.save(&image_path) {
                        panic!("Failed to save image: {}", e);
                    };
                }

                info!("Exiting renderer");
                app_exit_writer.send(AppExit::Success);
            }
        } else {
            // clears channel for skipped frames
            while receiver.try_recv().is_ok() {}
            scene_controller.state = SceneState::Render(n - 1);
        }
    }
}
