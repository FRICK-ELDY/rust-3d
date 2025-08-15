// render/src/renderer/targets/mod.rs
mod depth;
mod msaa;

pub use depth::DepthTarget;
pub use msaa::MsaaTargets;

use winit::dpi::PhysicalSize;
use crate::gpu::instance::GpuContext;

pub struct RenderTargets {
    pub depth: DepthTarget,
    pub msaa: Option<MsaaTargets>,
}

impl RenderTargets {
    pub fn new(
        ctx: &GpuContext,
        size: PhysicalSize<u32>,
        color_format: wgpu::TextureFormat,
        sample_count: u32,
    ) -> Self {
        let depth = DepthTarget::create(ctx, size, sample_count);
        let msaa = if sample_count > 1 {
            Some(MsaaTargets::create(ctx, size, color_format, sample_count))
        } else { None };
        Self { depth, msaa }
    }

    pub fn resize(&mut self, ctx: &GpuContext, size: PhysicalSize<u32>, color_format: wgpu::TextureFormat, sample_count: u32) {
        self.depth = DepthTarget::create(ctx, size, sample_count);
        self.msaa = if sample_count > 1 {
            Some(MsaaTargets::create(ctx, size, color_format, sample_count))
        } else { None };
    }
}
