use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use instant::Instant;
use hecs::*;

mod dynamics;
mod input;
mod renderer;
mod web;

use input::*;
use dynamics::*;
use renderer::*;
use web::*;

struct SpaceShip {
    angle: f64
}

struct EndZone {
    width: f64,
    height: f64,
}

fn system_finish(world: &mut World) {
    let mut show_win = false;
    for (_id, position) in &mut world.query::<With<SpaceShip, &Position>>() {
        for (_id, (zone_limits, zone_position)) in &mut world.query::<(&EndZone, &Position)>(){
            if zone_position.x+20.0 < position.x && position.x < zone_position.x+zone_limits.width-20.0 {
                if zone_position.y+20.0 < position.y && position.y < zone_position.y + zone_limits.height-20.0 {
                    show_win = true;
                }
            }
        }
    }
    if show_win {
        let renderer = Renderer::text(String::from("¡Victoria!"), "black", "20pt arial");
        let position = Position {
            x: 550.0,
            y: 200.0,
        };
        world.spawn((renderer, position));
    }
}

fn world_level_1(mut store: &mut HashMap<&'static str, web_sys::HtmlImageElement>) -> World {
    let mut world = World::new();
    {
        let mut renderer = Renderer::sprite("/space.png", &mut store);
        renderer.set_z(-100);
        renderer.set_fixed(true);
        let position = Position {
            x: 0.0,
            y: 0.0,
        };
        world.spawn((renderer, position));
    }
    {
        let renderer = Renderer::circle(100.0, "#ff0000");
        let position = Position {
            x: 650.0,
            y: 400.0,
        };
        let mass = Mass {
            mass: 5.0
        };
        world.spawn((renderer, position, mass));
    }
    {
        let renderer = Renderer::rect(50.0, 50.0, "rgba(102, 145, 209, 0.7)");
        let position = Position {
            x: 800.0-25.0,
            y: 400.0-25.0,
        };
        world.spawn((renderer, position));
    }
    {
        let renderer = Renderer::rect(100.0, 100.0, "rgba(250, 126, 55, 0.7)");
        let position = Position {
            x: 1100.0-50.0,
            y: 400.0-50.0,
        };
        let end = EndZone {
            width: 100.0,
            height: 100.0,
        };
        world.spawn((renderer, position, end));
    }
    {
        let renderer = Renderer::text(String::from("Desplázate hasta el cuadrado naranja"), "black", "16pt arial");
        let position = Position {
            x: 550.0,
            y: 100.0,
        };
        world.spawn((renderer, position));
    }
    {
        let renderer = Renderer::circle(20.0, "#00ff00");
        let position = Position {
            x: 800.0,
            y: 400.0,
        };
        let velocity = Velocity {
            x: 0.0,
            y: 500.0
        };
        let spaceship = SpaceShip {
            angle: 0.0
        };
        world.spawn((renderer, position, velocity, spaceship));
    }
    {
        let camera = Camera {
            offset: Position {
                x: 0.0,
                y: 0.0,
            }
        };
        world.spawn((camera,));
    }
    world
}

const KEY_W: u32 = 87;
const KEY_S: u32 = 83;

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

    let mut store = HashMap::new();
    let world = world_level_1(&mut store);

    let input = Rc::new(RefCell::new(Input::default()));
    let input_handler = input.clone();
    let click_handler = Closure::wrap(Box::new(move ||{
        let mut input = input_handler.borrow_mut();
        input.forward = !input.forward;
    }) as Box<dyn Fn()>);

    let input_handler = input.clone();
    let keydown_handler = Closure::wrap(Box::new(move |event: web_sys::Event|{
        let mut input = input_handler.borrow_mut();
        let event = wasm_bindgen::JsCast::dyn_ref::<web_sys::KeyboardEvent>(&event).unwrap();
        match event.key_code() {
            KEY_W => input.forward = true,
            KEY_S => input.brake = true,
            _ => ()
        };
    }) as Box<dyn Fn(_)>);

    let input_handler = input.clone();
    let keyup_handler = Closure::wrap(Box::new(move |event: web_sys::Event|{
        let mut input = input_handler.borrow_mut();
        let event = wasm_bindgen::JsCast::dyn_ref::<web_sys::KeyboardEvent>(&event).unwrap();
        match event.key_code() {
            KEY_W => input.forward = false,
            KEY_S => input.brake = false,
            _ => ()
        };
    }) as Box<dyn Fn(_)>);

    window().set_onkeydown(Some(keydown_handler.as_ref().unchecked_ref()));
    window().set_onkeyup(Some(keyup_handler.as_ref().unchecked_ref()));
    click_handler.forget();
    keydown_handler.forget();
    keyup_handler.forget();
    
    let ginput = input.clone();
    let now = Instant::now();
    request_animation_frame(move ||{
        gloop(context, world, ginput, store, now);
    });
}

pub fn gloop(context: web_sys::CanvasRenderingContext2d, world: World, input: Rc<RefCell<Input>>, store: HashMap<&'static str, web_sys::HtmlImageElement>, prev: Instant){
    let mut world = world;
    context.clear_rect(0.0, 0.0, 1360.0, 768.0);
    let now = Instant::now();
    let delta = now.duration_since(prev).as_secs_f64();
    {
        if delta < 0.1 {
            let input = input.borrow();

            system_spacecraft_input(&mut world, &input, delta);
            system_gravity(&mut world, delta);
            system_finish(&mut world);
            system_offset(&mut world);
            system_renderer(&mut world, &context, &store);
        }
    }
    request_animation_frame(move ||{
        gloop(context, world, input, store, now);
    });
}
