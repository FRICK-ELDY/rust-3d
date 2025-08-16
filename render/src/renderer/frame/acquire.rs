// render/src/renderer/frame/acquire.rs
use anyhow::Result;
use crate::gpu::surface::SurfaceState;
use crate::gpu::instance::GpuContext;

pub fn acquire_frame(
    surface: &mut SurfaceState,
    ctx: &GpuContext,
) -> Result<Option<wgpu::SurfaceTexture>> {
    surface.acquire_current_frame(ctx)
}
