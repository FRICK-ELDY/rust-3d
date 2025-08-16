// platform/web/src/lib.rs

// ── 非 wasm ビルド用のダミー（workspace 全体ビルドを壊さない）
#[cfg(not(target_arch = "wasm32"))]
pub fn init_for_native_builds_only() {}

// ── ここから wasm (Web) 専用実装
#[cfg(target_arch = "wasm32")]
mod web_app {
    use std::{cell::RefCell, rc::Rc};

    use wasm_bindgen::JsCast;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;

    use winit::dpi::PhysicalSize;
    use winit::platform::web::WindowAttributesExtWebSys;

    // ===== 任意：/config.toml を文字列で取る（使わないなら未使用抑制）=====
    #[allow(unused)]
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
    fn set_canvas_pixel_size(
        canvas: &web_sys::HtmlCanvasElement,
        win: &web_sys::Window,
    ) -> (u32, u32) {
        let dpr = win.device_pixel_ratio();

        // HtmlCanvasElement → Element に（get_bounding_client_rect は Element の API）
        let elem: &web_sys::Element = canvas.unchecked_ref();
        let rect = elem.get_bounding_client_rect();

        let w = (rect.width() * dpr).max(1.0).round() as u32;
        let h = (rect.height() * dpr).max(1.0).round() as u32;

        if canvas.width() != w {
            canvas.set_width(w);
        }
        if canvas.height() != h {
            canvas.set_height(h);
        }
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
            let canvas: web_sys::HtmlCanvasElement =
                doc.create_element("canvas").unwrap().dyn_into().unwrap();
            // キー入力のためにフォーカス可能に（tabindex）＋ 見た目のスタイル
            let _ = canvas.set_attribute("tabindex", "0");
            let _ = canvas.set_attribute(
                "style",
                "width:100%;height:100%;display:block;margin:0;padding:0;touch-action:none;outline:none;",
            );
            let _ = root.append_child(&canvas);

            // 初回の実ピクセル設定
            let (init_w, init_h) = set_canvas_pixel_size(&canvas, &win);

            // ===== winit: WindowAttributes 経由で Web Canvas に結び付け =====
            let event_loop = winit::event_loop::EventLoop::new().expect("EventLoop::new");
            let attrs = winit::window::Window::default_attributes()
                .with_title("rust-3d (web)")
                .with_canvas(Some(canvas.clone())); // ← Web固有

            // 0.30系の古い create_window を一時使用（行単位で警告抑制）
            #[allow(deprecated)]
            let window = event_loop.create_window(attrs).expect("create_window");

            // Surface<'static> が必要なら 'static に固定（ページ閉鎖で OS が解放）
            let window_static: &'static winit::window::Window = Box::leak(Box::new(window));

            // ===== Scene & Renderer 準備 =====
            // ここはあなたの API に合わせてください：
            // - Scene: render::scene::Scene::default()
            // - Renderer::new(&Window, PhysicalSize<u32>) -> impl Future<Output=Result<..>>
            let scene = render::scene::Scene::default();
            let renderer = render::Renderer::new(window_static, PhysicalSize::new(init_w, init_h))
                .await
                .expect("renderer init failed");

            let renderer_rc = Rc::new(RefCell::new(renderer));
            let scene_rc = Rc::new(scene); // 参照だけ渡すので Rc で OK

            // ===== リサイズ対応：ResizeObserver =====
            {
                let renderer_for_resize = renderer_rc.clone();
                let canvas_for_resize = canvas.clone();
                let win_for_resize = win.clone();

                let resize_cb = wasm_bindgen::closure::Closure::<
                    dyn FnMut(js_sys::Array, web_sys::ResizeObserver),
                >::wrap(Box::new(move |_entries, _observer| {
                    let (w, h) = set_canvas_pixel_size(&canvas_for_resize, &win_for_resize);
                    renderer_for_resize
                        .borrow_mut()
                        .resize(PhysicalSize::new(w, h));
                }));
                let ro = web_sys::ResizeObserver::new(resize_cb.as_ref().unchecked_ref()).unwrap();
                ro.observe(&canvas);
                resize_cb.forget(); // keep alive
            }

            // ===== キー入力（必要に応じてゲーム側へつなぐ） =====
            {
                // キャンバスにフォーカスを当てる（クリックで）
                let canvas_click = {
                    let c = canvas.clone();
                    wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::MouseEvent)>::wrap(
                        Box::new(move |_| {
                            let _ = c.focus(); // ignore error if cannot focus
                        }),
                    )
                };
                let _ = canvas.add_event_listener_with_callback(
                    "mousedown",
                    canvas_click.as_ref().unchecked_ref(),
                );
                canvas_click.forget();

                // キーボード（例：F キーで何かトグルするならここ）
                let keydown_cb =
                    wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::KeyboardEvent)>::wrap(
                        Box::new(move |_e: web_sys::KeyboardEvent| {
                            // 必要なら renderer_rc.borrow_mut().xxx();
                        }),
                    );
                let _ = doc.add_event_listener_with_callback(
                    "keydown",
                    keydown_cb.as_ref().unchecked_ref(),
                );
                keydown_cb.forget();
            }

            // ===== requestAnimationFrame ループ =====
            let raf_cell: Rc<RefCell<Option<wasm_bindgen::closure::Closure<dyn FnMut()>>>> =
                Rc::new(RefCell::new(None));
            let raf_closure = raf_cell.clone();
            let renderer_for_loop = renderer_rc.clone();
            let scene_for_loop = scene_rc.clone();

            *raf_closure.borrow_mut() =
                Some(wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                    if let Err(e) = renderer_for_loop.borrow_mut().render(&scene_for_loop) {
                        web_sys::console::error_1(&format!("render error: {:?}", e).into());
                    }
                    // 次フレーム
                    let _ = web_sys::window().unwrap().request_animation_frame(
                        raf_cell.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
                    );
                })
                    as Box<dyn FnMut()>));

            let _ = web_sys::window().unwrap().request_animation_frame(
                raf_closure
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .unchecked_ref(),
            );

            // 例：設定ファイルを読みたい場合（存在しなければ無視）
            // if let Some(toml_str) = fetch_text("/config.toml").await {
            //     // parse して renderer に反映など
            // }
        });

        Ok(())
    }
}

// wasm-bindgen はモジュール外に公開関数が必要なので re-export
#[cfg(target_arch = "wasm32")]
pub use web_app::start;
