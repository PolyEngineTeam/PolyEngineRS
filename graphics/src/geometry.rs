

use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::device::Device;


use std::sync::Arc;

pub use polyengine_core::*;
use crate::vertex::Vertex;

pub type GeometryId = u32;

pub struct Geometry {
    pub vertex_buffer : Vec<Arc<dyn vulkano::buffer::BufferAccess + Send + Sync>>
}

impl Geometry {
    pub fn from_data(device: Arc<Device>, data: &Vec<na::Vector3<FScalar>>) -> Self {
        let mapper = |v3: &na::Vector3<FScalar>|{ return Vertex{position: [v3.x as f32, v3.y as f32, v3.z as f32]}; };
        let map_handle = data.into_iter().map(mapper);
        
        let vertex_buffer = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, map_handle).unwrap();

        return Geometry{ vertex_buffer: vec![vertex_buffer] }
    }
}