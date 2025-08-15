// platform/desktop/src/main.rs
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
        Self {
            window: None,
            renderer: None,
            scene: Default::default(),
        }
    }
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

        // wgpu のため 'static 参照を作る（注意: ライフタイムは実質プロセス終了まで）
        let window_static: &'static Window =
            unsafe { std::mem::transmute::<&Window, &'static Window>(&*window) };

        // 非同期初期化をブロックして待つ
        let renderer =
            pollster::block_on(render::Renderer::new(window_static, size)).expect("renderer init failed");

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
        let Some(window) = &self.window else { return; };
        if window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                // 必要ならここでクリーンアップ
                std::process::exit(0);
            }
            WindowEvent::Resized(new_size) => {
                if let Some(r) = &mut self.renderer {
                    // 0 サイズは無視（最小化対策）
                    if new_size.width > 0 && new_size.height > 0 {
                        r.resize(PhysicalSize::new(new_size.width, new_size.height));
                    }
                }
                window.request_redraw();
            }
            WindowEvent::ScaleFactorChanged { scale_factor: _, mut inner_size_writer } => {
                // 現在の物理解像度を取り直し、同じサイズを要求（DPI変更に追従）
                let sz = window.inner_size();
                let _ = inner_size_writer.request_inner_size(sz);
                if let Some(r) = &mut self.renderer {
                    if sz.width > 0 && sz.height > 0 {
                        r.resize(PhysicalSize::new(sz.width, sz.height));
                    }
                }
                window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                if let Some(r) = &mut self.renderer {
                    // SurfaceError は Renderer 側でハンドリング済み
                    let _ = r.render(&self.scene);
                }
                // 継続描画したい場合は request_redraw を投げ続ける
                window.request_redraw();
            }
            _ => {}
        }
    }

    // アイドル時にも描画を回したい場合（省電力したいなら削除可）
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(w) = &self.window {
            w.request_redraw();
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().expect("failed to create event loop");
    let mut app = App::default();
    event_loop.run_app(&mut app).expect("event loop terminated unexpectedly");
}
