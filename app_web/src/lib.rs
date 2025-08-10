use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use winit::platform::web::WindowBuilderExtWebSys;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    wasm_bindgen_futures::spawn_local(async move {
        let win = web_sys::window().expect("no window");
        let doc = win.document().expect("no document");
        let root = doc.get_element_by_id("app").expect("#app not found");

        let canvas: web_sys::HtmlCanvasElement = doc
            .create_element("canvas").unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        canvas.set_attribute("style", "width:100%;height:100%;display:block;").ok();
        root.append_child(&canvas).ok();

        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        let window = winit::window::WindowBuilder::new()
            .with_title("rust-3d (web)")
            .with_canvas(Some(canvas))
            .build(&event_loop)
            .unwrap();

        let window_static: &'static winit::window::Window = Box::leak(Box::new(window));

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = instance.create_surface(window_static).unwrap();

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .expect("No adapter");

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
            },
            None,
        )
        .await
        .expect("request_device failed");

        let size = window_static.inner_size();
        let caps = surface.get_capabilities(&adapter);
        let format = caps.formats[0];
        let mut config = wgpu::SurfaceConfiguration {
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

        let mut game = game::GameState::new();

        use std::cell::RefCell;
        use std::rc::Rc;

        let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            game.update(1.0 / 60.0);

            let frame = match surface.get_current_texture() {
                Ok(f) => f,
                Err(_) => {
                    let size = window_static.inner_size();
                    config.width = size.width.max(1);
                    config.height = size.height.max(1);
                    surface.configure(&device, &config);
                    return;
                }
            };

            let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

            let t = game.t.sin() * 0.5 + 0.5;
            {
                let _rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color { r: t as f64, g: 0.07, b: 0.12, a: 1.0 }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });
            }

            queue.submit([encoder.finish()]);
            frame.present();

            web_sys::window().unwrap()
                .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .unwrap();
        }) as Box<dyn FnMut()>));

        web_sys::window().unwrap()
            .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    });

    Ok(())
}
