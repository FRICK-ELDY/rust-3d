// render/src/lib.rs
use anyhow::{anyhow, Result};
use winit::dpi::PhysicalSize;

pub struct Renderer {
    surface: wgpu::Surface<'static>,
    device:  wgpu::Device,
    queue:   wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    clear_color: [f32; 4],
}

impl Renderer {
    pub async fn new(
        window: &'static winit::window::Window,
        size: PhysicalSize<u32>,
    ) -> Result<Self> {
        // Instance（Web は WebGL2、Desktop は自動選択）
        #[cfg(target_arch = "wasm32")]
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::GL,
            ..Default::default()
        });
        #[cfg(not(target_arch = "wasm32"))]
        let instance = wgpu::Instance::default();

        // Surface
        let surface = instance.create_surface(window).expect("create surface");

        // Adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| anyhow!("No adapter"))?;

        // Limits（Web は WebGL2 互換、Desktop は downlevel 既定）
        #[cfg(target_arch = "wasm32")]
        let limits = wgpu::Limits::downlevel_webgl2_defaults();
        #[cfg(not(target_arch = "wasm32"))]
        let limits = wgpu::Limits::downlevel_defaults();

        // Device / Queue（wasm だと RequestDeviceError が Send/Sync ではないため明示変換）
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: limits,
                },
                None,
            )
            .await
            .map_err(|e| anyhow!("request_device failed: {e:?}"))?;

        // Surface 設定
        let caps = surface.get_capabilities(&adapter);
        let format = caps.formats[0];
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            clear_color: [0.02, 0.07, 0.12, 1.0], // 既定の背景色
        })
    }

    /// 背景色を RGBA（0.0..1.0）で設定
    pub fn set_clear_color(&mut self, rgba: [f32; 4]) {
        self.clear_color = rgba;
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(&mut self, game: &mut core::GameState) -> Result<()> {
        // 簡易 dt
        game.update(1.0 / 60.0);

        // フレーム取得（失敗時は再設定だけしてスキップ）
        let frame = match self.surface.get_current_texture() {
            Ok(f) => f,
            Err(_) => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
        };

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        // 背景色：コンフィグの色に軽く脈動を足す
        let base = self.clear_color;
        let pulse = (game.t.sin() * 0.2).max(-0.2); // ちょい変化
        let clear = wgpu::Color {
            r: (base[0] + pulse) as f64,
            g: base[1] as f64,
            b: base[2] as f64,
            a: base[3] as f64,
        };

        {
            let _rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        self.queue.submit([encoder.finish()]);
        frame.present();
        Ok(())
    }
}
