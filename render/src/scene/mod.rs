mod camera;
mod material;
mod mesh;
mod primitives;

pub use camera::Camera;

#[derive(Default)]
pub struct Scene {
    pub camera: Camera,
}
