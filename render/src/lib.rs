pub mod config;
pub mod gpu;
pub mod scene;       // Camera/Mesh/Material など
pub mod passes;
pub mod renderer; 

pub use renderer::Renderer;
pub use scene::{Camera}; // 必要に応じて他も re-export
