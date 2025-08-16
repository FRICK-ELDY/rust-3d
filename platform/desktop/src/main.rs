// platform/desktop/src/main.rs
use std::sync::Arc;
use winit::keyboard::{Key, KeyCode, NamedKey, PhysicalKey};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    renderer: Option<render::Renderer>,
    scene: render::scene::Scene,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title("rust-3d (desktop)"))
                .expect("failed to create window"),
        );

        let size = window.inner_size();

        // wgpu のため 'static 参照を作る
        let window_static: &'static Window =
            unsafe { std::mem::transmute::<&Window, &'static Window>(&*window) };

        // 非同期初期化を同期で待つ
        let renderer = pollster::block_on(render::Renderer::new(window_static, size))
            .expect("renderer init failed");

        self.renderer = Some(renderer);
        self.window = Some(window);

        // 初回描画を依頼
        if let Some(w) = &self.window {
            w.request_redraw();
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = &self.window else {
            return;
        };
        if window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                std::process::exit(0);
            }

            WindowEvent::Resized(new_size) => {
                if let Some(r) = &mut self.renderer
                    && new_size.width > 0
                    && new_size.height > 0
                {
                    r.resize(PhysicalSize::new(new_size.width, new_size.height));
                }
                window.request_redraw();
            }

            WindowEvent::ScaleFactorChanged {
                scale_factor: _,
                mut inner_size_writer,
            } => {
                // DPI変更に追従
                let sz = window.inner_size();
                let _ = inner_size_writer.request_inner_size(sz);
                if let Some(r) = &mut self.renderer
                    && sz.width > 0
                    && sz.height > 0
                {
                    r.resize(PhysicalSize::new(sz.width, sz.height));
                }
                window.request_redraw();
            }

            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    let is_f1_named = matches!(event.logical_key, Key::Named(NamedKey::F1));
                    let is_f1_physical = event.physical_key == PhysicalKey::Code(KeyCode::F1);
                    if (is_f1_named || is_f1_physical)
                        && let Some(r) = &mut self.renderer
                    {
                        let on = r.toggle_overlay();
                        println!("[overlay] {}", if on { "ON" } else { "OFF" });
                        window.request_redraw(); // 反映を速く見るため即リドロー
                    }
                }
            }

            WindowEvent::RedrawRequested => {
                if let Some(r) = &mut self.renderer {
                    let _ = r.render(&self.scene); // SurfaceError は内部で処理
                }
                // 継続描画したいなら投げ続ける
                window.request_redraw();
            }

            _ => {}
        }
    }

    // アイドル時にも描画。省電力化したいなら削除OK
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(w) = &self.window {
            w.request_redraw();
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().expect("failed to create event loop");
    let mut app = App::default();
    event_loop
        .run_app(&mut app)
        .expect("event loop terminated unexpectedly");
}
