use std::rc::Rc;
use std::cell::RefCell;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use hecs::*;

mod web;
mod renderer;

use web::*;
use renderer::*;

fn system_renderer(world: &mut World, context: &web_sys::CanvasRenderingContext2d) {
    for (_id, visual) in &mut world.query::<&Visual>() {
        visual.painter.paint(&context);
    }
}

fn world_level_1() -> World {
    let mut world = World::new();
    {
        let renderer = CircleRenderer {
            position: Position {
                x: 100.0,
                y: 60.0,
            },
            radius: 10.0
        };
        let visual = Visual {
            painter: Box::new(renderer)
        };
        world.spawn((visual,));
    }
    {
        let renderer = RectRenderer {
            position: Position {
                x: 100.0,
                y: 100.0
            },
            width: 40.0,
            height: 500.0
        };
        let visual = Visual {
            painter: Box::new(renderer)
        };
        world.spawn((visual,));
    }
    {
        let renderer = SpriteRenderer::new("/comuneros.jpg", Position { x: 300.0, y: 300.0});
        let visual = Visual {
            painter: Box::new(renderer)
        };
        world.spawn((visual,));
    }
    world
}

#[derive(Default)]
pub struct Input {
    click: bool,
}

#[wasm_bindgen(start)]
pub fn start() {
    let canvas = document().get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let world = world_level_1();

    let input = Rc::new(RefCell::new(Input::default()));
    let input_handler = input.clone();
    let click_handler = Closure::wrap(Box::new(move ||{
        let mut input = input_handler.borrow_mut();
        input.click = !input.click;
    }) as Box<dyn Fn()>);

    window().set_onclick(Some(click_handler.as_ref().unchecked_ref()));
    click_handler.forget();
    
    let ginput = input.clone();
    request_animation_frame(move ||{
        gloop(context, world, ginput);
    });
}

pub fn gloop(context: web_sys::CanvasRenderingContext2d, world: World, input: Rc<RefCell<Input>>){
    let mut world = world;
    context.clear_rect(0.0, 0.0, 150.0, 150.0);
    {
        let input = input.borrow();

        if input.click {
            context.set_fill_style(&JsValue::from_str("#FF0000"));
        } else {
            context.set_fill_style(&JsValue::from_str("#000000"));
        }

        system_renderer(&mut world, &context);
    }
    request_animation_frame(move ||{
        gloop(context, world, input);
    });
}
