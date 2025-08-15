// render/src/renderer/frame/acquire.rs
use anyhow::{Result, anyhow};
use winit::dpi::PhysicalSize;
use crate::gpu::surface::SurfaceState;
use crate::gpu::instance::GpuContext;

pub fn acquire_frame<'a>(
    surface: &'a mut SurfaceState,
    size: PhysicalSize<u32>,
    _ctx: &GpuContext,
) -> Result<Option<wgpu::SurfaceTexture>> {
    match surface.surface.get_current_texture() {
        Ok(frame) => Ok(Some(frame)),
        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
            surface.resize(_ctx, size);
            Ok(None)
        }
        Err(wgpu::SurfaceError::Timeout) => Ok(None),
        Err(wgpu::SurfaceError::Other) => {
            eprintln!("[wgpu] SurfaceError::Other");
            Ok(None)
        }
        Err(wgpu::SurfaceError::OutOfMemory) => Err(anyhow!("wgpu SurfaceError::OutOfMemory")),
    }
}
