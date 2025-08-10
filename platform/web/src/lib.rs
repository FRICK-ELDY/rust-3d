// platform/web/src/lib.rs
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use winit::platform::web::WindowBuilderExtWebSys;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // パニック時にブラウザのコンソールへスタックトレース
    console_error_panic_hook::set_once();

    // 非同期初期化（WASMは async main が使えないため）
    wasm_bindgen_futures::spawn_local(async move {
        // ===== DOM 準備 =====
        let win = web_sys::window().unwrap();
        let doc = win.document().unwrap();
        let root = doc.get_element_by_id("app").unwrap();

        // Canvas を作成して #app に追加
        let canvas: web_sys::HtmlCanvasElement = doc
            .create_element("canvas").unwrap()
            .dyn_into().unwrap();
        canvas.set_attribute("style", "width:100%;height:100%;display:block;").ok();
        root.append_child(&canvas).ok();

        // ===== winit Window を Canvas に紐付け =====
        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        let window = winit::window::WindowBuilder::new()
            .with_title("rust-3d (web)")
            .with_canvas(Some(canvas))
            .build(&event_loop)
            .unwrap();

        // Surface<'static> 要件に合わせて 'static に固定
        let window_static: &'static winit::window::Window = Box::leak(Box::new(window));
        let size = window_static.inner_size();

        // ===== 共通ロジック & レンダラー =====
        let mut game = core::GameState::new();
        let mut renderer = render::Renderer::new(window_static, size)
            .await
            .expect("renderer");

        // ===== requestAnimationFrame ループ =====
        use std::cell::RefCell;
        use std::rc::Rc;
        let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let _ = renderer.render(&mut game); // エラーは握りつぶし（ログに出したければ map_err 等で）

            // 次フレーム
            web_sys::window().unwrap()
                .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .unwrap();
        }) as Box<dyn FnMut()>));

        // 1フレーム目を開始
        web_sys::window().unwrap()
            .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    });

    Ok(())
}
