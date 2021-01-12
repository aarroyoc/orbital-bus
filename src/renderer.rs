use std::f64;
use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::web::*;
use crate::dynamics::*;

type ImageStore = HashMap<&'static str, web_sys::HtmlImageElement>;

type Color = &'static str;

pub trait Paintable: Send + Sync + 'static {
    fn paint(&self, context: &web_sys::CanvasRenderingContext2d, position: &Position, store: &ImageStore);
}

pub struct CircleRenderer {
    pub radius: f64,
    pub color: Color,
}

impl Paintable for CircleRenderer {
    fn paint(&self, context: &web_sys::CanvasRenderingContext2d, position: &Position, _store: &ImageStore) {
        context.begin_path();

        context.set_fill_style(&JsValue::from_str(self.color));

        context
            .arc(position.x, position.y, self.radius, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        context.fill();
    }
}

pub struct RectRenderer {
    pub width: f64,
    pub height: f64,
    pub color: Color,
}

impl Paintable for RectRenderer {
    fn paint(&self, context: &web_sys::CanvasRenderingContext2d, position: &Position, _store: &ImageStore) {
        context.set_fill_style(&JsValue::from_str(self.color));
        context.fill_rect(position.x, position.y, self.width, self.height);
    }
}

pub struct SpriteRenderer {
    image: &'static str,
}

impl SpriteRenderer {
    pub fn new(url: &'static str, store: &mut ImageStore) -> Self {
        let img = document().create_element("img")
            .expect("Unable to create img element")
            .dyn_into::<web_sys::HtmlImageElement>()
            .unwrap();
        img.set_src(url);
        store.insert(url, img);
        SpriteRenderer {
            image: url
        }
    }
}

impl Paintable for SpriteRenderer {
    fn paint(&self, context: &web_sys::CanvasRenderingContext2d, position: &Position, store: &ImageStore) {
        let image = store.get(self.image);
        context.draw_image_with_html_image_element(&image.unwrap(), position.x, position.y).unwrap();
    }
}

type Font = &'static str;

pub struct TextRenderer {
    pub text: String,
    pub color: Color,
    pub font: Font,
}

impl Paintable for TextRenderer {
    fn paint(&self, context: &web_sys::CanvasRenderingContext2d, position: &Position, _store: &ImageStore) {
        context.set_fill_style(&JsValue::from_str(self.color));
        context.set_font(self.font);
        context.fill_text(&self.text, position.x, position.y).unwrap();
    }
}

pub struct Visual {
    pub painter: Box<dyn Paintable>,
}

impl Visual {
    pub fn from_paintable(renderer: Box<dyn Paintable>) -> Self {
        Visual {
            painter: renderer
        }
    }
}