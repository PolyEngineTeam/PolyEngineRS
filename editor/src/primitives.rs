use polyengine_core::*;

pub fn generate_box(scale: FScalar) -> Vec<na::Vector3<FScalar>> {
    return vec![
        na::Vector3::<FScalar>::new(-0.5, -0.5, 0.0) * scale,
        na::Vector3::<FScalar>::new(0.5, 0.5, 0.0) * scale,
        na::Vector3::<FScalar>::new(0.5, -0.5, 0.0) * scale,
        na::Vector3::<FScalar>::new(-0.5, -0.5, 0.0) * scale,
        na::Vector3::<FScalar>::new(-0.5, 0.5, 0.0) * scale,
        na::Vector3::<FScalar>::new(0.5, 0.5, 0.0) * scale,
    ];
}
