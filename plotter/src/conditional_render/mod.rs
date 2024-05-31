use bevy::{
    ecs::system::Resource, 
    render::camera::RenderTarget
};

// Render target (window or headless image) will be stored in this resource
#[derive(Resource)]
pub struct ConditionalRenderTarget {
    pub target: Option<RenderTarget>
}
impl ConditionalRenderTarget {
    pub fn new(value: RenderTarget) -> ConditionalRenderTarget {
        ConditionalRenderTarget { target: Some(value) }
    }
}

// Empty system used purely for scheduling conditional rendering systems
pub fn setup_conditional_rendering() {}
