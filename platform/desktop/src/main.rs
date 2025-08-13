use anyhow::Result;
use std::{fs, path::PathBuf};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn load_config_desktop() -> game::GameConfig {
    let exe_dir = std::env::current_exe().ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    let path = exe_dir.join("config.toml");
    if let Ok(txt) = fs::read_to_string(&path) {
        if let Ok(cfg) = game::GameConfig::from_toml_str(&txt) {
            return cfg;
        }
    }
    game::GameConfig::default()
}

fn main() -> Result<()> {
    let cfg = load_config_desktop();

    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("rust-3d (desktop)")
        .with_inner_size(winit::dpi::LogicalSize::new(cfg.window_width, cfg.window_height))
        .with_fullscreen(if cfg.fullscreen {
            Some(winit::window::Fullscreen::Borderless(None))
        } else {
            None
        })
        .build(&event_loop)?;

    let window_static: &'static winit::window::Window = Box::leak(Box::new(window));
    let size = window_static.inner_size();

    let mut game = game::GameState::new();
    let mut renderer = pollster::block_on(render::Renderer::new(window_static, size))?;

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::Resized(s) => renderer.resize(s),
                _ => {}
            },
            Event::AboutToWait => {
                renderer.set_clear_color(cfg.clear_color);
                let _ = renderer.render(&mut game);
            }
            _ => {}
        }
    })?;
    Ok(())
}
