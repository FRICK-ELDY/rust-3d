// render/src/renderer/renderer.rs
use anyhow::{anyhow, Result};
use winit::dpi::PhysicalSize;

use crate::config::RenderConfig;
use crate::gpu::{instance::GpuContext, surface::SurfaceState};
use crate::passes::Pass;
use crate::scene::Scene;

use super::frame::{acquire_frame, begin_main_pass};
use super::pass_manager::PassManager;
use super::state::RenderState;
use super::targets::RenderTargets;

use crate::debug::fps::FpsCounter;
use crate::debug::mini_text::MiniText;
use crate::debug::overlay::UiOverlay;

pub struct Renderer {
    ctx: GpuContext,
    surface: SurfaceState,
    pub config: RenderConfig,

    state: RenderState,
    targets: RenderTargets,
    passes: PassManager,

    // debug overlay
    fps: FpsCounter,
    text: MiniText,
    ui: UiOverlay,
    overlay_enabled: bool,
}

impl Renderer {
    pub async fn new(
        window: &'static winit::window::Window,
        size: PhysicalSize<u32>,
    ) -> Result<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window)?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
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
            })
            .await?;

        // device/queue を ctx にムーブ
        let ctx = GpuContext {
            instance,
            adapter,
            device,
            queue,
        };

        // SurfaceState は GpuContext から作る
        let surface_state = SurfaceState::new(&ctx, window, size)?;

        // レンダリング論理状態
        let msaa4 = cfg!(feature = "msaa4");
        let state = RenderState::new(size, msaa4);

        // 各種ターゲット（MSAA/Depth）
        let targets =
            RenderTargets::new(&ctx, size, surface_state.config.format, state.sample_count);

        let info = ctx.adapter.get_info();
        println!("[wgpu] adapter={} backend={:?}", info.name, info.backend);

        // ---- Debug Overlay 初期化 ----
        let tex_w = 256u32;
        let tex_h = 64u32;
        // device は ctx にムーブ済み。以降は ctx.device / ctx.queue を使う
        let ui = UiOverlay::new(&ctx.device, surface_state.format, tex_w, tex_h);
        let text = MiniText::new(tex_w, tex_h);
        let fps = FpsCounter::new();

        Ok(Self {
            ctx,
            surface: surface_state,
            config: RenderConfig::default(),
            state,
            targets,
            passes: PassManager::default(),
            ui,
            text,
            fps,
            overlay_enabled: true,
        })
    }

    pub fn set_clear_color(&mut self, rgba: [f32; 4]) {
        self.state.set_clear_color(rgba);
    }

    pub fn add_pass<P: Pass + Send + Sync + 'static>(&mut self, pass: P) {
        self.passes.add(pass);
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }
        self.state.size = new_size;

        // Surface を再設定
        self.surface.resize(&self.ctx, new_size);

        // 付随ターゲットも再作成
        let fmt = self.surface.config.format;
        self.targets
            .resize(&self.ctx, self.state.size, fmt, self.state.sample_count);
    }

    pub fn overlay_enabled(&self) -> bool {
        self.overlay_enabled
    }

    pub fn set_overlay_enabled(&mut self, enabled: bool) {
        self.overlay_enabled = enabled;
    }

    pub fn toggle_overlay(&mut self) -> bool {
        self.overlay_enabled = !self.overlay_enabled;
        self.overlay_enabled
    }

    pub fn render(&mut self, _scene: &Scene) -> Result<()> {
        // 先頭で FPS 計測更新
        self.fps.tick();

        // フレーム取得（ゼロサイズ時はスキップ）
        let Some(frame) = acquire_frame(&mut self.surface, &self.ctx)? else {
            return Ok(());
        };
        let swap_view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("render/encoder"),
            });

        // MSAA 有無で描画先を分岐
        let (color_view, resolve_target) = if let Some(msaa) = &self.targets.msaa {
            (&msaa.color_view, Some(&swap_view))
        } else {
            (&swap_view, None)
        };

        // メインパス（クリア＋各 Pass 実行）
        {
            let mut rpass = begin_main_pass(
                &mut encoder,
                color_view,
                resolve_target,
                &self.targets.depth.view,
                self.state.clear_color,
            );
            self.passes.draw_all(&mut rpass);
        }

        if self.overlay_enabled {
            // 表示テキスト（複数行）
            let w = self.surface.config.width;
            let h = self.surface.config.height;
            let mut s = String::new();
            use std::fmt::Write as _;
            let _ = write!(
                s,
                "FPS:  {:.1}\nFrame: {:.2} ms\nSurface: {} x {}",
                self.fps.fps, self.fps.frame_ms, w, h
            );

            // ★ 必要幅の見積もり（最長行の文字数を使う & 見出し "PERFORMANCE" も考慮）
            let mut max_chars = "PERFORMANCE".len() as u32;
            for line in s.lines() {
                max_chars = max_chars.max(line.chars().count() as u32);
            }
            // 6px ピッチ + 左右マージン（左右各4px＝合計8px）
            let needed_w = (max_chars * 6) + 8;
            // 上限は任意（512など）にクランプ
            let new_w = needed_w.clamp(self.ui.tex_w, 512);

            // ★ 先に GPU / CPU の幅を合わせる（ここが重要）
            if new_w > self.ui.tex_w {
                self.ui.ensure_width(&self.ctx.device, new_w);
                self.text.resize(new_w, self.text.tex_h);
            }

            // CPU上で描画 → GPUへアップロード → 合成
            self.text.clear();
            self.text
                .draw_text(4, 6, "PERFORMANCE", [255, 255, 255, 180]);
            self.text.draw_text(4, 18, &s, [230, 230, 230, 255]);
            self.ui.upload_rgba(&self.ctx.queue, self.text.as_rgba());

            // 最終カラーターゲット（＝Present する swap_view）に合成
            self.ui.draw(&mut encoder, &swap_view, w, h);
        }

        self.ctx.queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(())
    }
}
