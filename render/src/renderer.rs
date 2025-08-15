use anyhow::{anyhow, Result};
use winit::dpi::PhysicalSize;

use crate::config::RenderConfig;
use crate::gpu::{instance::GpuContext, surface::SurfaceState};
use crate::scene::Scene;

/// 深度テクスチャ一式（将来シャドウ等のサンプリング用途も見据えてサンプラを保持）
struct DepthTarget {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
    format: wgpu::TextureFormat,
    sample_count: u32,
}

impl DepthTarget {
    const DEFAULT_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    fn create(
        device: &wgpu::Device,
        width: u32,
        height: u32,
        format: wgpu::TextureFormat,
        sample_count: u32,
    ) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("render/depth-texture"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("render/depth-sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            // 将来的に深度比較サンプル（シャドウマップ等）にも使える設定
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });
        Self { texture, view, sampler, format, sample_count }
    }
}

pub struct Renderer {
    ctx: GpuContext,
    surface: SurfaceState,
    pub config: RenderConfig,
    clear_color: wgpu::Color,

    // 追加: ウィンドウサイズと深度ターゲット/MSAA情報
    size: PhysicalSize<u32>,
    depth: DepthTarget,
    sample_count: u32, // featureで切替可能にしておく（今は1）
}

impl Renderer {
    pub async fn new(
        window: &'static winit::window::Window,
        size: PhysicalSize<u32>,
    ) -> Result<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let surface  = instance.create_surface(window)?;

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .map_err(|e| anyhow!("wgpu: request_adapter failed: {e:?}"))?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("render/device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::Performance,
                ..Default::default()
            }).await?;

        let ctx = GpuContext { instance, adapter, device, queue };
        let surface_state = SurfaceState::new(&ctx, window, size)?;

        // MSAA準備（featureで切替）: デフォルト1、将来 "msaa4" を有効にしたら4に
        let sample_count: u32 = if cfg!(feature = "msaa4") { 4 } else { 1 };

        // 深度ターゲット生成（フォーマットは固定でOK。必要ならSurface側と合わせる）
        let depth = DepthTarget::create(
            &ctx.device,
            size.width.max(1),
            size.height.max(1),
            DepthTarget::DEFAULT_FORMAT,
            sample_count,
        );

        let info = ctx.adapter.get_info();
        println!("[wgpu] adapter={} backend={:?}", info.name, info.backend);

        Ok(Self {
            ctx,
            surface: surface_state,
            config: RenderConfig::default(),
            clear_color: wgpu::Color { r: 0.06, g: 0.07, b: 0.09, a: 1.0 },
            size,
            depth,
            sample_count,
        })
    }

    pub fn set_clear_color(&mut self, rgba: [f32; 4]) {
        self.clear_color = wgpu::Color {
            r: rgba[0] as f64,
            g: rgba[1] as f64,
            b: rgba[2] as f64,
            a: rgba[3] as f64,
        };
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 {
            // 最小1x1にしておくか、ゼロは無視（ここでは無視に）
            return;
        }
        self.size = new_size;
        // Surfaceを再設定（内部でSurfaceConfigurationを更新する想定）
        self.surface.resize(&self.ctx, new_size);
        // 深度ターゲットを作り直す
        self.depth = DepthTarget::create(
            &self.ctx.device,
            self.size.width,
            self.size.height,
            self.depth.format,
            self.sample_count,
        );
        // MSAAカラーターゲットを使う場合はここで再生成する（今は未使用）
    }

    pub fn render(&mut self, _scene: &Scene) -> Result<()> {
        // get_current_texture のエラーを個別に握り、自己復帰を試みる
        let frame = match self.surface.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(err) => {
                match err {
                    wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated => {
                        // Surfaceが失われた/古くなった: 再設定してフレームをスキップ
                        self.surface.resize(&self.ctx, self.size);
                        return Ok(());
                    }
                    wgpu::SurfaceError::Timeout => {
                        // 軽微: このフレームはスキップ
                        return Ok(());
                    }
                    wgpu::SurfaceError::OutOfMemory => {
                        // 深刻: エラーで返す
                        return Err(anyhow!("wgpu SurfaceError::OutOfMemory"));
                    }
                    wgpu::SurfaceError::Other => {
                        // たまに来る曖昧系。ログってスキップで復帰を試みる
                        eprintln!("[wgpu] SurfaceError::Other");
                        return Ok(());
                    }
                }
            }
        };

        let swap_view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("render/encoder"),
        });

        // 将来MSAAを有効化する場合:
        // - msaa_color_tex / msaa_color_view をここで用意し
        // - color_attachment.view = &msaa_color_view
        // - color_attachment.resolve_target = Some(&swap_view)
        // - 深度も sample_count=4 で作成済み
        let color_attachment = wgpu::RenderPassColorAttachment {
            view: &swap_view,
            resolve_target: None,
            depth_slice: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(self.clear_color),
                store: wgpu::StoreOp::Store,
            },
        };

        let rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("render/main-pass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // --- ここに実際の描画呼び出し（パイプライン/バインド/メッシュ描画など）を追加 ---
        // 例: self.draw_grid(&mut rpass);
        drop(rpass);

        self.ctx.queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(())
    }
}
