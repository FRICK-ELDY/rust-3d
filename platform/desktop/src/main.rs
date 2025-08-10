use anyhow::Result;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("rust-3d (desktop)")
        .build(&event_loop)?;

    // ★ ここで 'static にする（Surface<'static> 用）
    let window_static: &'static winit::window::Window = Box::leak(Box::new(window));
    let size = window_static.inner_size();

    let mut game = core::GameState::new();
    let mut renderer = pollster::block_on(render::Renderer::new(window_static, size))?;

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::Resized(s) => renderer.resize(s),
                _ => {}
            },
            Event::AboutToWait => {
                let _ = renderer.render(&mut game);
            }
            _ => {}
        }
    })?;
    Ok(())
}
