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

pub fn set_local_storage(key: &str, value: &str) {
    let storage = window().local_storage().expect("no 'localStorage' found").unwrap();
    storage.set_item(key, value).unwrap();
}

pub fn get_local_storage(key: &str) -> String {
    let storage = window().local_storage().expect("no 'localStorage' found").unwrap();
    storage.get_item(key).unwrap().unwrap()
}

pub fn go_web(url: &str) {
    window().location().set_href(url).expect("error in 'window.location.href'");
}