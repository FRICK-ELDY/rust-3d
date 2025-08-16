// platform/web/src/lib.rs
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use winit::dpi::PhysicalSize;
use winit::platform::web::WindowAttributesExtWebSys;

// ===== 任意：/config.toml を文字列で取る（使わないなら削除OK）=====
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
    // panic をブラウザコンソールへ
    console_error_panic_hook::set_once();

    // 非同期初期化 & ループ
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
        // キー入力のためにフォーカス可能に（tabindex）＋ 見た目のスタイル
        canvas.set_attribute("tabindex", "0").ok();
        canvas.set_attribute(
            "style",
            "width:100%;height:100%;display:block;margin:0;padding:0;touch-action:none;outline:none;",
        ).ok();
        root.append_child(&canvas).ok();

        // 初回の実ピクセル設定
        let (init_w, init_h) = set_canvas_pixel_size(&canvas, &win);

        // ===== winit: WindowAttributes 経由で Web Canvas に結び付け =====
        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        let attrs = winit::window::Window::default_attributes()
            .with_title("rust-3d (web)")
            .with_canvas(Some(canvas.clone())); // ← Web固有
        let window = event_loop.create_window(attrs).unwrap();

        // Surface<'static> が必要なのでリークで 'static に固定（ページ閉鎖で解放）
        let window_static: &'static winit::window::Window = Box::leak(Box::new(window));

        // ===== Scene & Renderer 準備 =====
        let scene = render::scene::Scene::default();
        let renderer = render::Renderer::new(
            window_static,
            PhysicalSize::new(init_w, init_h),
        )
        .await
        .expect("renderer init failed");

        // ---- 共有 Rc<RefCell<..>> へ包む ----
        use std::cell::RefCell;
        use std::rc::Rc;
        let renderer_rc = Rc::new(RefCell::new(renderer));

        // ===== リサイズ対応：ResizeObserver =====
        {
            let renderer_for_resize = renderer_rc.clone();
            let canvas_for_resize = canvas.clone();
            let win_for_resize = win.clone();

            let resize_cb =
                Closure::<dyn FnMut(js_sys::Array, web_sys::ResizeObserver)>::wrap(Box::new(
                    move |_, _| {
                        let (w, h) = set_canvas_pixel_size(&canvas_for_resize, &win_for_resize);
                        renderer_for_resize
                            .borrow_mut()
                            .resize(PhysicalSize::new(w, h));
                    },
                ));
            let ro = web_sys::ResizeObserver::new(resize_cb.as_ref().unchecked_ref()).unwrap();
            ro.observe(&canvas);
            resize_cb.forget(); // keep alive
        }

        // ===== キー入力: F1 でオーバーレイ ON/OFF =====
        {
            // キャンバスにフォーカスを当てる（クリックで）
            let canvas_click = {
                let c = canvas.clone();
                Closure::<dyn FnMut(web_sys::MouseEvent)>::wrap(Box::new(move |_| {
                    let _ = c.focus(); // ignore error if cannot focus
                }))
            };
            canvas
                .add_event_listener_with_callback("mousedown", canvas_click.as_ref().unchecked_ref())
                .ok();
            canvas_click.forget();

            // ドキュメント全体で F1 を拾う（ブラウザヘルプを開かないように阻止）
            let renderer_for_key = renderer_rc.clone();
            let keydown_cb = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::wrap(Box::new(
                move |e: web_sys::KeyboardEvent| {
                    if e.key() == "F1" {
                        e.prevent_default(); // ブラウザのヘルプ抑制
                        let on = renderer_for_key.borrow_mut().toggle_overlay();
                        web_sys::console::log_1(
                            &format!("[overlay] {}", if on { "ON" } else { "OFF" }).into(),
                        );
                    }
                },
            ));
            doc.add_event_listener_with_callback("keydown", keydown_cb.as_ref().unchecked_ref())
                .ok();
            keydown_cb.forget();
        }

        // ===== requestAnimationFrame ループ =====
        let scene_rc = Rc::new(scene); // &Scene で渡すので共有でOK
        let raf_cell: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
        let raf_closure = raf_cell.clone();
        let renderer_for_loop = renderer_rc.clone();
        let scene_for_loop = scene_rc.clone();

        *raf_closure.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            if let Err(e) = renderer_for_loop.borrow_mut().render(&scene_for_loop) {
                web_sys::console::error_1(&format!("render error: {:?}", e).into());
            }
            // 次フレーム
            let _ = web_sys::window()
                .unwrap()
                .request_animation_frame(
                    raf_cell.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
                );
        }) as Box<dyn FnMut()>));

        web_sys::window()
            .unwrap()
            .request_animation_frame(
                raf_closure.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
            )
            .unwrap();

        // 例：任意の設定を読みたい場合（存在しなければ無視）
        // if let Some(toml_str) = fetch_text("/config.toml").await {
        //     // 必要ならここで parse して renderer.set_clear_color([...]) など
        // }
    });

    Ok(())
}
