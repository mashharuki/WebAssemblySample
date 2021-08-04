mod utils;
mod logic;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::ImageData;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

// コンソール出力のマクロ
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()));
}

// 時間計測のマクロ
macro_rules! measure_elapsed_time {
    ($t:tt, $s:block) => {{
        let window = web_sys::window().expect("should hava a window in this context");
        let performance = window.performance().expect("performance should be available");
        let start = performance.now();
        let result = { $s };
        let end = performance.now();
        console_log!("{}:{}[ms]", $t, end - start);
    }};
}

// 描画と時間計測をwasm側で実施するための関数
#[wasm_bindgen]
pub fn draw_mandelbrot_set() {
    // キャンバスID
    const CANVAS_ID: &str = "canvas_wasm";
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(CANVAS_ID).unwrap();
    // HtmlCanvasElement型のAPIを利用するためにElement型からキャストする。
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().map_err(|_| ()).unwrap();
    // Object型からCanvasRenderingContext2d型にキャストする。
    let context = canvas.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
    // 縦幅と横幅の情報を取得する。
    let canvas_w = canvas.width() as usize;
    let canvas_h = canvas.width() as usize;
    // それぞれ変数を設定
    const X_MIN: f64 = -1.5;
    const X_MAX: f64 = 0.5;
    const Y_MIN: f64 = -1.0;
    const Y_MAX: f64 = 1.0;
    const MAX_ITER: usize = 64;

    // 時間を計測するためのマクロを呼び出す。
    let mut result = measure_elapsed_time!("generate:wasm¥telapsed:", {
        // 色情報を生成する。
        logic::generate_mandelbrot_set(canvas_w, canvas_h, X_MIN, X_MAX, Y_MIN, Y_MAX, MAX_ITER);
    });
    // 再び時間計測用のマクロを呼び出す。
    measure_elapsed_time!("draw:wasm¥telapsed:", {
        // 画像データ生成
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut result),
            canvas.width(),
            canvas.height(),
        );
        if let Ok(data) = data {
            let _ = context.put_image_data(&data, 0.0, 0.0);
        } 
    });
}