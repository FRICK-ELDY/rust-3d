use anyhow::{anyhow, Result};
use winit::dpi::PhysicalSize;
use crate::gpu::{instance::GpuContext, surface::SurfaceState};

/// SurfaceTexture を取得。自己復帰できるものは None でスキップ。
pub fn acquire_frame(surface: &mut SurfaceState, size: PhysicalSize<u32>, ctx: &GpuContext)
    -> Result<Option<wgpu::SurfaceTexture>>
{
    match surface.surface.get_current_texture() {
        Ok(frame) => Ok(Some(frame)),
        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
            surface.resize(ctx, size);
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

/// メインレンダーパスを開始して返す（MSAA あり/なし両対応）
pub fn begin_main_pass<'a>(
    encoder: &'a mut wgpu::CommandEncoder,
    color_view: &'a wgpu::TextureView,
    resolve_target: Option<&'a wgpu::TextureView>,
    depth_view: &'a wgpu::TextureView,
    clear: wgpu::Color,
) -> wgpu::RenderPass<'a> {
    let color_attachment = wgpu::RenderPassColorAttachment {
        view: color_view,
        resolve_target,
        depth_slice: None,
        ops: wgpu::Operations { load: wgpu::LoadOp::Clear(clear), store: wgpu::StoreOp::Store },
    };

    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("render/main-pass"),
        color_attachments: &[Some(color_attachment)],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: depth_view,
            depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }),
            stencil_ops: None,
        }),
        timestamp_writes: None,
        occlusion_query_set: None,
    })
}
