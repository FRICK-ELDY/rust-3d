use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};

struct App {
    window: Option<Arc<Window>>,
    renderer: Option<render::Renderer>,
    scene: render::scene::Scene,
}

impl Default for App {
    fn default() -> Self {
        Self { window: None, renderer: None, scene: Default::default() }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() { return; }
        let window = Arc::new(
            event_loop.create_window(Window::default_attributes().with_title("wgpu test")).unwrap()
        );
        let size = window.inner_size();
        let window_static: &'static Window = unsafe { std::mem::transmute::<&Window, &'static Window>(&window) };

        // 非同期初期化をブロックして待つ
        let renderer = pollster::block_on(render::Renderer::new(window_static, size)).unwrap();

        self.window = Some(window);
        self.renderer = Some(renderer);
    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, window_id: winit::window::WindowId, event: WindowEvent) {
        let Some(window) = &self.window else { return; };
        if window.id() != window_id { return; }

        match event {
            WindowEvent::CloseRequested => {
                std::process::exit(0);
            }
            WindowEvent::Resized(new_size) => {
                if let Some(r) = &mut self.renderer {
                    r.resize(PhysicalSize::new(new_size.width, new_size.height));
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(r) = &mut self.renderer {
                    let _ = r.render(&self.scene);
                }
                window.request_redraw();
            }
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
