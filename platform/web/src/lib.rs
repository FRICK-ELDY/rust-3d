use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use winit::platform::web::WindowBuilderExtWebSys;
use winit::dpi::PhysicalSize;

// ===== 任意：/config.toml を文字列で取る =====
async fn fetch_text(url: &str) -> Option<String> {
    let win = web_sys::window()?;
    let resp_val = JsFuture::from(win.fetch_with_str(url)).await.ok()?;
    let resp: web_sys::Response = resp_val.dyn_into().ok()?;
    if !resp.ok() {
        web_sys::console::warn_1(&format!("fetch {} -> status {}", url, resp.status()).into());
        return None;
    }
    JsFuture::from(resp.text().ok()?).await.ok()?.as_string()
}

// ===== Canvas の実ピクセル幅高さを DPR で設定 =====
fn set_canvas_pixel_size(canvas: &web_sys::HtmlCanvasElement, win: &web_sys::Window) -> (u32, u32) {
    let dpr = win.device_pixel_ratio();
    let rect = canvas.get_bounding_client_rect();
    let w = (rect.width() * dpr).max(1.0).round() as u32;
    let h = (rect.height() * dpr).max(1.0).round() as u32;
    canvas.set_width(w);
    canvas.set_height(h);
    (w, h)
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    // wasm_logger を使うなら:
    // wasm_logger::init(wasm_logger::Config::default());

    wasm_bindgen_futures::spawn_local(async move {
        // ===== DOM 準備 =====
        let win = web_sys::window().expect("no window");
        let doc = win.document().expect("no document");
        let root = doc
            .get_element_by_id("app")
            .or_else(|| doc.body().map(|b| b.into()))
            .expect("#app or <body> not found");

        // Canvas 作成 & 追加（CSS サイズは100%）
        let canvas: web_sys::HtmlCanvasElement = doc
            .create_element("canvas").unwrap()
            .dyn_into().unwrap();
        canvas
            .set_attribute("style", "width:100%;height:100%;display:block;margin:0;padding:0;")
            .ok();
        root.append_child(&canvas).ok();

        // 初回の実ピクセル設定
        let (init_w, init_h) = set_canvas_pixel_size(&canvas, &win);

        // ===== winit Window を Canvas に紐付け =====
        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        let window = winit::window::WindowBuilder::new()
            .with_title("rust-3d (web)")
            .with_canvas(Some(canvas.clone()))
            .build(&event_loop)
            .unwrap();

        // Surface<'static> に合わせてリーク（Webでは終了時開放不要）
        let window_static: &'static winit::window::Window = Box::leak(Box::new(window));

        // ===== 設定ファイルの任意読込 =====
        let cfg = if let Some(toml_str) = fetch_text("/config.toml").await {
            game::GameConfig::from_toml_str(&toml_str).unwrap_or_default()
        } else {
            game::GameConfig::default()
        };

        // ===== GameState & Renderer 準備 =====
        let game = game::GameState::new();
        let mut renderer = render::Renderer::new(window_static, (init_w, init_h).into())
            .await
            .expect("renderer init failed");
        renderer.set_clear_color(cfg.clear_color);

        // ===== リサイズ対応：ResizeObserver =====
        // Renderer を Rc<RefCell<..>> に包んで、コールバックから触れるようにする
        use std::cell::RefCell;
        use std::rc::Rc;
        let renderer_rc = Rc::new(RefCell::new(renderer));
        let renderer_for_resize = renderer_rc.clone();
        let canvas_for_resize = canvas.clone();
        let win_for_resize = win.clone();

        let resize_cb = Closure::<dyn FnMut(js_sys::Array, web_sys::ResizeObserver)>::wrap(Box::new(
            move |_, _| {
                let (w, h) = set_canvas_pixel_size(&canvas_for_resize, &win_for_resize);
                renderer_for_resize.borrow_mut().resize(PhysicalSize::new(w, h));
            },
        ));
        let ro = web_sys::ResizeObserver::new(resize_cb.as_ref().unchecked_ref()).unwrap();
        ro.observe(&canvas);
        resize_cb.forget(); // keep alive

        // ===== requestAnimationFrame ループ =====
        let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
        let g = f.clone();

        // game はループ内で可変参照が必要 → Rc<RefCell>
        let game_rc = Rc::new(RefCell::new(game));
        let renderer_for_loop = renderer_rc.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            // 1フレーム描画
            if let Err(e) = renderer_for_loop.borrow_mut().render(&mut *game_rc.borrow_mut()) {
                web_sys::console::error_1(&format!("render error: {:?}", e).into());
            }
            // 次フレーム
            web_sys::window()
                .unwrap()
                .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .unwrap();
        }) as Box<dyn FnMut()>));

        web_sys::window()
            .unwrap()
            .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    });

    Ok(())
}
