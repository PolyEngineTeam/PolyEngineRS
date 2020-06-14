use vulkano::framebuffer::{FramebufferAbstract, RenderPassAbstract};

use std::{sync::Arc, vec::Vec};

pub struct RenderTarget {
    pub framebuffers: Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,
    pub render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
}
