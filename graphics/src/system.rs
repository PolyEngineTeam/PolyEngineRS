use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::instance::Instance;

use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::WindowId;

use std::sync::Arc;
use std::vec::Vec;

use crate::context::RenderContext;
use crate::error::RenderingError;
use crate::renderer::Renderer;
use crate::vertex::Vertex;

use crate::GeometryId;

use polyengine_core::*;

pub struct RenderingSystem {
    #[allow(dead_code)] // TODO remove this
    instance: Arc<Instance>,
    context: RenderContext,
    renderer: Renderer,

    // TEMPORARY
    #[allow(dead_code)]
    vertex_buffer: Vec<Arc<dyn vulkano::buffer::BufferAccess + Send + Sync>>,
}

impl RenderingSystem {
    pub fn new(elwt: &EventLoopWindowTarget<()>) -> Self {
        // Instance
        let instance = Instance::new(None, &vulkano_win::required_extensions(), None)
            .expect("failed to create instance");

        let context = RenderContext::new(elwt, instance.clone());
        let renderer = Renderer::new(
            context.device.clone(),
            context.default_window_render_pass.clone(),
        );

        // TEMPORARY BEGIN
        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            context.device.clone(),
            BufferUsage::all(),
            false,
            [
                Vertex {
                    position: [-0.5, -0.25, 0.0],
                },
                Vertex {
                    position: [0.0, 0.5, 0.0],
                },
                Vertex {
                    position: [0.25, -0.1, 0.0],
                },
            ]
            .iter()
            .cloned(),
        )
        .unwrap();
        // TEMPORARY END

        return RenderingSystem {
            instance,
            context,
            renderer,

            vertex_buffer: vec![vertex_buffer],
        };
    }

    pub fn open_window(&mut self, elwt: &EventLoopWindowTarget<()>, window_name: &str) -> WindowId {
        return self.context.create_window(elwt, window_name);
    }

    pub fn window_resized(&mut self, window_id: WindowId, _new_size: PhysicalSize<u32>) {
        self.context
            .windows
            .get_mut(&window_id)
            .unwrap()
            .on_resize();
    }

    pub fn close_window(&mut self, window_id: WindowId) -> bool {
        return match self.context.close_window(window_id) {
            Ok(_) => self.context.window_count() == 0,
            Err(_) => false,
        };
    }

    pub fn create_geometry(&mut self, data: &Vec<na::Vector3<FScalar>>) -> GeometryId {
        return self.context.create_geometry(data);
    }

    pub fn end_frame(&mut self) {
        for window in self.context.windows.values_mut() {
            let (image_num, acquire_future) = match window.acquire_next_image() {
                Ok(r) => r,
                Err(RenderingError::ImageAcquireFailed) => {
                    return;
                }
                Err(RenderingError::RecreateSwapchainFailed) => {
                    return;
                }
                Err(e) => panic!("Acquire failed! {:?}", e),
            };

            let g = &self.context.geometries.values().next().unwrap();
            window.draw(image_num, acquire_future, &self.renderer, g);
        }
    }
}
