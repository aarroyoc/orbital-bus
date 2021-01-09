use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn document() -> web_sys::Document {
    window().document().expect("no document")
}

pub fn request_animation_frame<F>(f: F)
where
    F: 'static + FnOnce()
{
    let closure = Closure::once_into_js(f)
                    .dyn_into::<js_sys::Function>()
                    .unwrap();
    window()
        .request_animation_frame(&closure)
        .expect("should register `requestAnimationFrame` OK");
}