//! summary: デスクトップ最小ループ（共通 Renderer を利用）
//! path: platform/desktop/src/lib.rs

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
}

impl<'w> GpuState<'w> {
    async fn new(window: &'w Window) -> Result<Self> {
        // Instance と Surface は platform 側で作る
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            flags: wgpu::InstanceFlags::empty(),
            ..Default::default()
        });
        let surface = instance.create_surface(window).context("create_surface")?;
        let size = window.inner_size();

        // 共通初期化
        let init = render::init_with_surface(
            &instance,
            surface,
            (size.width, size.height),
            render::RenderInitOptions::default(),
        ).await?;

        // アダプタ情報ログ
        let info = init.adapter_info;
        println!(
            "Adapter: {} | backend={:?} | type={:?} | vendor=0x{:04x} | device=0x{:04x}",
            info.name, info.backend, info.device_type, info.vendor, info.device
        );

        Ok(Self { renderer: init.renderer, size })
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 { return; }
        self.size = new_size;
        self.renderer.resize(new_size.width, new_size.height);
    }

    fn render(&mut self) -> Result<()> {
        let clear = wgpu::Color { r: 0.07, g: 0.10, b: 0.18, a: 1.0 };
        self.renderer.render_clear(clear)
    }
}

struct App {
    window: Option<&'static Window>,
    gpu:    Option<GpuState<'static>>,
}

impl App { fn new() -> Self { Self { window: None, gpu: None } } }

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let attrs = WindowAttributes::default()
                .with_title("platform_desktop::run()")
                .with_inner_size(PhysicalSize::new(1280, 720));

            let window = event_loop.create_window(attrs).expect("create window");
            let window_static: &'static Window = Box::leak(Box::new(window));
            self.window = Some(window_static);

            let gpu = pollster::block_on(GpuState::new(window_static)).expect("init wgpu");
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

/// デスクトップ用の最小「クリア描画」ランナー
pub fn run() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;
    Ok(())
}
