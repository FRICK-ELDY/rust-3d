use super::instance::GpuContext;
use anyhow::{anyhow, Result};
use winit::dpi::PhysicalSize;

pub struct SurfaceState {
    pub surface: wgpu::Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,
    pub format: wgpu::TextureFormat,
}

impl SurfaceState {
    pub fn new(
        ctx: &GpuContext,
        window: &'static winit::window::Window,
        size: PhysicalSize<u32>,
    ) -> Result<Self> {
        let surface = ctx.instance.create_surface(window)?;

        let caps = surface.get_capabilities(&ctx.adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&ctx.device, &config);

        Ok(Self {
            surface,
            config,
            format,
        })
    }

    pub fn resize(&mut self, ctx: &GpuContext, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 {
            self.config.width = 0;
            self.config.height = 0;
            return;
        }
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&ctx.device, &self.config);
    }

    /// Lost/Outdated の復旧（サイズは self.config に従う）
    pub fn reconfigure(&mut self, ctx: &GpuContext) {
        if self.config.width == 0 || self.config.height == 0 {
            return;
        }
        self.surface.configure(&ctx.device, &self.config);
    }

    /// 現在のフレームを**堅牢に**取得する:
    /// - ゼロサイズ: Ok(None)
    /// - Timeout: device.poll 後 1 回だけ再試行
    /// - Lost/Outdated: reconfigure 後 1 回だけ再試行
    /// - OutOfMemory: Err（致命扱い）
    pub fn acquire_current_frame(
        &mut self,
        ctx: &GpuContext,
    ) -> Result<Option<wgpu::SurfaceTexture>> {
        if self.config.width == 0 || self.config.height == 0 {
            return Ok(None);
        }

        match self.surface.get_current_texture() {
            Ok(frame) => Ok(Some(frame)),
            Err(wgpu::SurfaceError::Timeout) => {
                #[cfg(not(target_arch = "wasm32"))]
                std::thread::yield_now();

                self.surface
                    .get_current_texture()
                    .map(Some)
                    .map_err(|e| anyhow!("SurfaceError after Timeout retry: {e:?}"))
            }
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                self.reconfigure(ctx);
                self.surface
                    .get_current_texture()
                    .map(Some)
                    .map_err(|e| anyhow!("SurfaceError after reconfigure retry: {e:?}"))
            }
            Err(wgpu::SurfaceError::OutOfMemory) => Err(anyhow!("wgpu SurfaceError::OutOfMemory")),
            // ← これを追加（フォールバック）
            Err(other) => Err(anyhow!("wgpu SurfaceError (other): {other:?}")),
        }
    }
}
