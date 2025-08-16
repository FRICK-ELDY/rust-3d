// render/src/renderer/frame/acquire.rs
use crate::gpu::instance::GpuContext;
use crate::gpu::surface::SurfaceState;
use anyhow::Result;

pub fn acquire_frame(
    surface: &mut SurfaceState,
    ctx: &GpuContext,
) -> Result<Option<wgpu::SurfaceTexture>> {
    surface.acquire_current_frame(ctx)
}
