mod camera;
mod mesh;
mod material;
mod primitives;

pub use camera::Camera;

pub struct Scene {
    pub camera: Camera,
}
impl Default for Scene {
    fn default() -> Self { Self { camera: Camera::default() } }
}
