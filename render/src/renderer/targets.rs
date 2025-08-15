use winit::dpi::PhysicalSize;
use crate::gpu::depth_target::DepthTarget;
use crate::gpu::msaa::{MsaaTargets, new_if_needed};
use crate::gpu::instance::GpuContext;

pub struct RenderTargets {
    pub depth: DepthTarget,
    pub msaa: Option<MsaaTargets>,
    pub sample_count: u32,
}

impl RenderTargets {
    pub fn new(ctx: &GpuContext, size: PhysicalSize<u32>, surface_format: wgpu::TextureFormat, sample_count: u32) -> Self {
        let depth = DepthTarget::new(
            &ctx.device,
            size.width.max(1),
            size.height.max(1),
            DepthTarget::DEFAULT_FORMAT,
            sample_count,
        );
        let msaa = new_if_needed(&ctx.device, size.width, size.height, surface_format, sample_count);
        Self { depth, msaa, sample_count }
    }

    pub fn resize(&mut self, ctx: &GpuContext, size: PhysicalSize<u32>, surface_format: wgpu::TextureFormat) {
        self.depth.resize(&ctx.device, size.width, size.height);
        match &mut self.msaa {
            Some(msaa) => msaa.resize(&ctx.device, size.width, size.height),
            None => {
                self.msaa = new_if_needed(&ctx.device, size.width, size.height, surface_format, self.sample_count);
            }
        }
    }
}
