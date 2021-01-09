use std::f64;
use std::cell::RefCell;
use std::collections::HashMap;

use wasm_bindgen::JsCast;

use crate::web::*;

pub trait Paintable: Send + Sync + 'static {
    fn paint(&self, context: &web_sys::CanvasRenderingContext2d);
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
    fn paint(&self, context: &web_sys::CanvasRenderingContext2d) {
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
    fn paint(&self, context: &web_sys::CanvasRenderingContext2d) {
        context.fill_rect(self.position.x, self.position.y, self.width, self.height);
    }
}

thread_local! {
    static SPRITES: RefCell<HashMap<&'static str, web_sys::HtmlImageElement>> = RefCell::new(HashMap::new());
}

pub struct SpriteRenderer {
    pub position: Position,
    image: &'static str,
}

impl SpriteRenderer {
    pub fn new(url: &'static str, position: Position) -> Self {
        let img = document().create_element("img")
            .expect("Unable to create img element")
            .dyn_into::<web_sys::HtmlImageElement>()
            .unwrap();
        img.set_src(url);
        SPRITES.with(|sprites|{
            let mut sprites = sprites.borrow_mut();
            sprites.insert(url, img);
        });
        SpriteRenderer {
            position,
            image: url
        }
    }
}

impl Paintable for SpriteRenderer {
    fn paint(&self, context: &web_sys::CanvasRenderingContext2d) {
        SPRITES.with(|sprites|{
            let sprites = sprites.borrow();
            let image = sprites.get(self.image);
            context.draw_image_with_html_image_element(&image.unwrap(), self.position.x, self.position.y).unwrap();
        });
    }
}

pub struct TextRenderer {
    pub text: String,
}

pub struct Visual {
    pub painter: Box<dyn Paintable>,
}