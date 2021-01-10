use std::f64;
use std::collections::HashMap;

use wasm_bindgen::JsCast;

use crate::web::*;

type ImageStore = HashMap<&'static str, web_sys::HtmlImageElement>;

pub trait Paintable: Send + Sync + 'static {
    fn paint(&self, context: &web_sys::CanvasRenderingContext2d, store: &ImageStore);
}

pub struct Position {
    pub x: f64,
    pub y: f64,
}

pub struct CircleRenderer {
    pub position: Position,
    pub radius: f64,
}

impl Paintable for CircleRenderer {
    fn paint(&self, context: &web_sys::CanvasRenderingContext2d, _store: &ImageStore) {
        context.begin_path();

        context
            .arc(self.position.x, self.position.y, self.radius, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        context.fill();
    }
}

pub struct RectRenderer {
    pub position: Position,
    pub width: f64,
    pub height: f64,
}

impl Paintable for RectRenderer {
    fn paint(&self, context: &web_sys::CanvasRenderingContext2d, _store: &ImageStore) {
        context.fill_rect(self.position.x, self.position.y, self.width, self.height);
    }
}

pub struct SpriteRenderer {
    pub position: Position,
    image: &'static str,
}

impl SpriteRenderer {
    pub fn new(url: &'static str, position: Position, store: &mut ImageStore) -> Self {
        let img = document().create_element("img")
            .expect("Unable to create img element")
            .dyn_into::<web_sys::HtmlImageElement>()
            .unwrap();
        img.set_src(url);
        store.insert(url, img);
        SpriteRenderer {
            position,
            image: url
        }
    }
}

impl Paintable for SpriteRenderer {
    fn paint(&self, context: &web_sys::CanvasRenderingContext2d, store: &ImageStore) {
        let image = store.get(self.image);
        context.draw_image_with_html_image_element(&image.unwrap(), self.position.x, self.position.y).unwrap();
    }
}

pub struct TextRenderer {
    pub text: String,
}

pub struct Visual {
    pub painter: Box<dyn Paintable>,
}