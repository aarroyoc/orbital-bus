use std::f64;
use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use hecs::{World, With};
use itertools::Itertools;

use crate::SpaceShip;
use crate::web::*;
use crate::dynamics::*;

type ImageStore = HashMap<&'static str, web_sys::HtmlImageElement>;
type Color = &'static str;
type Font = &'static str;

pub struct Camera {
    pub offset: Position
}

pub enum Renderer {
    CircleRenderer {
        radius: f64,
        color: Color,
        z: i32,
        fixed: bool,  
    },
    RectRenderer {
        width: f64,
        height: f64,
        color: Color,
        z: i32,
        fixed: bool,
    },
    TextRenderer {
        text: String,
        color: Color,
        font: Font,
        z: i32,
        fixed: bool,
    },
    SpriteRenderer {
        image: &'static str,
        z: i32,
        fixed: bool,
    }
}

impl Renderer {
    pub fn circle(radius: f64, color: Color) -> Self {
        Renderer::CircleRenderer {
            radius,
            color,
            z: 0,
            fixed: false
        }
    }
    pub fn rect(width: f64, height: f64, color: Color) -> Self {
        Renderer::RectRenderer {
            width,
            height,
            color,
            z: 0,
            fixed: false
        }
    }
    pub fn text(text: String, color: Color, font: Font) -> Self {
        Renderer::TextRenderer {
            text,
            color,
            font,
            z: 0,
            fixed: false,
        }
    }
    pub fn sprite(url: &'static str, store: &mut ImageStore) -> Self {
        let img = document().create_element("img")
            .expect("Unable to create img element")
            .dyn_into::<web_sys::HtmlImageElement>()
            .unwrap();
        img.set_src(url);
        store.insert(url, img);
        Renderer::SpriteRenderer {
            image: url,
            z: 0,
            fixed: false
        }
    }
    pub fn set_z(&mut self, new_z: i32) {
        match self {
            Renderer::CircleRenderer{z, ..} => *z = new_z,
            Renderer::RectRenderer{z, ..} => *z = new_z,
            Renderer::TextRenderer{z, ..} => *z = new_z,
            Renderer::SpriteRenderer{z, ..} => *z = new_z,
        }
    }
    pub fn get_z(&self) -> i32 {
        match self {
            Renderer::CircleRenderer{z, ..} => *z,
            Renderer::RectRenderer{z, ..} => *z,
            Renderer::TextRenderer{z, ..} => *z,
            Renderer::SpriteRenderer{z, ..} => *z,
        }
    }
    pub fn set_fixed(&mut self, new_fixed: bool) {
        match self {
            Renderer::CircleRenderer{fixed, ..} => *fixed = new_fixed,
            Renderer::RectRenderer{fixed, ..} => *fixed = new_fixed,
            Renderer::TextRenderer{fixed, ..} => *fixed = new_fixed,
            Renderer::SpriteRenderer{fixed, ..} => *fixed = new_fixed,
        }
    }
    pub fn is_fixed(&self) -> bool {
        match self {
            Renderer::CircleRenderer{fixed, ..} => *fixed,
            Renderer::RectRenderer{fixed, ..} => *fixed,
            Renderer::TextRenderer{fixed, ..} => *fixed,
            Renderer::SpriteRenderer{fixed, ..} => *fixed,
        }
    }
    fn paint(&self, context: &web_sys::CanvasRenderingContext2d, position: &Position, store: &ImageStore) {
        match self {
            Renderer::CircleRenderer { radius, color, ..} => {
                context.begin_path();

                context.set_fill_style(&JsValue::from_str(color));
        
                context
                    .arc(position.x, position.y, *radius, 0.0, f64::consts::PI * 2.0)
                    .unwrap();
        
                context.fill();
            },
            Renderer::RectRenderer { width, height, color, ..} => {
                context.set_fill_style(&JsValue::from_str(color));
                context.fill_rect(position.x, position.y, *width, *height);
            },
            Renderer::TextRenderer {color, font, text, .. } => {
                context.set_fill_style(&JsValue::from_str(*color));
                context.set_font(*font);
                context.fill_text(text, position.x, position.y).unwrap();
            },
            Renderer::SpriteRenderer { image, ..} => {
                let image = store.get(image);
                context.draw_image_with_html_image_element(&image.unwrap(), position.x, position.y).unwrap();
            }
        }
    }
}

pub fn system_offset(world: &mut World) {
    for (_id, position) in &mut world.query::<With<SpaceShip, &Position>>() {
        for (_id, camera) in &mut world.query::<&mut Camera>() {
            if position.x + camera.offset.x < 100.0 {
                camera.offset.x = 100.0 - position.x;
            }
            if position.y + camera.offset.y < 100.0 {
                camera.offset.y = 100.0 - position.y;
            }
            if position.x + camera.offset.x > 1260.0 {
                camera.offset.x = 1260.0 - position.x;
            }
            if position.y + camera.offset.y > 668.0 {
                camera.offset.y = 668.0 - position.y;
            }
        }
    }
}

pub fn system_renderer<'a>(world: &mut World, context: &web_sys::CanvasRenderingContext2d, store: &HashMap<&'static str, web_sys::HtmlImageElement>) {
    for (_id, camera) in &mut world.query::<&Camera>() {
        world.query::<(&Renderer, &Position)>()
            .iter()
            .sorted_by_key(|(_id, (renderer, _position))| {
                renderer.get_z()
            }).for_each(|(_id, (renderer, position))|{
                if renderer.is_fixed() {
                    renderer.paint(&context, &position, &store);
                } else {
                    let position = Position {
                        x: position.x + camera.offset.x,
                        y: position.y + camera.offset.y,
                    };
                    renderer.paint(&context, &position, &store);
                }
        });
    }
}