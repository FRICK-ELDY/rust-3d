pub mod config;
mod renderer;
mod gpu;
pub mod scene;       // Camera/Mesh/Material など

pub use renderer::Renderer;
pub use scene::{Camera}; // 必要に応じて他も re-export
