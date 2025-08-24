//! summary: デスクトップ最小ループ（共通 Renderer を利用, AppBuilder からの引数に対応）
//! path: platform/desktop/src/lib.rs

#![cfg(not(target_arch = "wasm32"))]

use anyhow::{Context, Result};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

struct GpuState<'w> {
    renderer: render::Renderer<'w>,
    size:     PhysicalSize<u32>,
    clear:    wgpu::Color,
}

impl<'w> GpuState<'w> {
    async fn new(window: &'w Window, prefer_high_perf: bool, clear: [f32;4]) -> Result<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            flags: wgpu::InstanceFlags::empty(),
            ..Default::default()
        });
        let surface = instance.create_surface(window).context("create_surface")?;
        let size = window.inner_size();

        let init = render::init_with_surface(
            &instance,
            surface,
            (size.width, size.height),
            render::RenderInitOptions { prefer_high_performance: prefer_high_perf },
        ).await?;

        // 任意: アダプタ情報ログ
        let info = init.adapter_info;
        println!(
            "Adapter: {} | backend={:?} | type={:?} | vendor=0x{:04x} | device=0x{:04x}",
            info.name, info.backend, info.device_type, info.vendor, info.device
        );

        Ok(Self {
            renderer: init.renderer,
            size,
            // ★ f32 → f64 へキャスト
            clear: wgpu::Color { r: clear[0] as f64, g: clear[1] as f64, b: clear[2] as f64, a: clear[3] as f64 },
        })
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 { return; }
        self.size = new_size;
        self.renderer.resize(new_size.width, new_size.height);
    }

    fn render(&mut self) -> Result<()> {
        self.renderer.render_clear(self.clear)
    }
}

struct App {
    window: Option<&'static Window>,
    gpu:    Option<GpuState<'static>>,
    prefer_high_perf: bool,
    initial_size: Option<(u32,u32)>,
    clear: [f32;4],
}

impl App {
    fn new(prefer_high_perf: bool, initial_size: Option<(u32,u32)>, clear: [f32;4]) -> Self {
        Self { window: None, gpu: None, prefer_high_perf, initial_size, clear }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let (w, h) = self.initial_size.unwrap_or((1280, 720));
            let attrs = WindowAttributes::default()
                .with_title("platform_desktop::run_with()")
                .with_inner_size(PhysicalSize::new(w, h));

            let window = event_loop.create_window(attrs).expect("create window");
            let window_static: &'static Window = Box::leak(Box::new(window));
            self.window = Some(window_static);

            let gpu = pollster::block_on(GpuState::new(
                window_static,
                self.prefer_high_perf,
                self.clear,
            )).expect("init wgpu");
            self.gpu = Some(gpu);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, mut event: WindowEvent) {
        match (self.window, self.gpu.as_mut()) {
            (Some(window), Some(gpu)) => match event {
                WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::Resized(new_size) => gpu.resize(new_size),
                WindowEvent::ScaleFactorChanged { scale_factor: _, ref mut inner_size_writer } => {
                    let _ = inner_size_writer.request_inner_size(gpu.size);
                }
                WindowEvent::RedrawRequested => {
                    if let Err(e) = gpu.render() {
                        eprintln!("render error: {e:?}");
                        event_loop.exit();
                    }
                }
                _ => { let _ = window; }
            },
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window { window.request_redraw(); }
    }
}

/// AppBuilder から呼ばれる統一入口（★ engine が呼ぶ関数）
pub fn run_with(
    clear_color: [f32;4],
    prefer_high_performance: bool,
    initial_size: Option<(u32,u32)>,
) -> Result<()> {
    let event_loop = EventLoop::new()?;
    let mut app = App::new(prefer_high_performance, initial_size, clear_color);
    event_loop.run_app(&mut app)?;
    Ok(())
}

/// 既定値で起動したいときのショートカット（任意）
pub fn run() -> Result<()> {
    run_with([0.07,0.10,0.18,1.0], true, Some((1280,720)))
}
