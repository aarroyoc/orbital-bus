use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::time::{Duration};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use instant::Instant;
use hecs::*;

mod web;
mod renderer;
mod dynamics;

use web::*;
use renderer::*;
use dynamics::*;

const MAX_SPEED: f64 = 200.0;

struct SpaceShip {
    angle: f64
}

fn system_renderer(world: &mut World, context: &web_sys::CanvasRenderingContext2d, store: &HashMap<&'static str, web_sys::HtmlImageElement>) {
    for (_id, (visual, position)) in &mut world.query::<(&Visual, &Position)>() {
        visual.painter.paint(&context, &position, &store);
    }
}

fn system_gravity(world: &mut World, delta: Duration) {
    let ms = delta.as_secs_f64();

    for (_id, (position, velocity)) in &mut world.query::<(&mut Position, &mut Velocity)>() {
        for (_id, (mass_position, mass)) in &mut world.query::<(&Position, &Mass)>() {
            velocity.x += (mass_position.x-position.x)*ms*mass.mass*0.0001;
            velocity.y += (mass_position.y-position.y)*ms*mass.mass*0.0001;
        }
        // Roce
        //velocity.x *= 0.99;
        //velocity.y *= 0.99;
        position.x += velocity.x * ms;
        position.y += velocity.y * ms;
    }
}

fn system_spacecraft_input(world: &mut World, input: &Input, delta: Duration) {
    let ms = delta.as_secs_f64();
    // with spacecraft to select only spacecraft
    for (_id, (spaceship, velocity)) in &mut world.query::<(&mut SpaceShip, &mut Velocity)>() {
        spaceship.angle = velocity.y.atan2(velocity.x);
        let mut x = velocity.x;
        let mut y = velocity.y;
        if input.forward {
            x += 100.0*ms*spaceship.angle.cos();
            y += 100.0*ms*spaceship.angle.sin();
        }
        if input.brake {
            x -= 100.0*ms*spaceship.angle.cos();
            y -= 100.0*ms*spaceship.angle.sin();
        }
        let speed = (x * x + y * y).sqrt();
        if speed < MAX_SPEED {
            velocity.x = x;
            velocity.y = y;
        }
    }
    
}

fn world_level_1(mut _store: &mut HashMap<&'static str, web_sys::HtmlImageElement>) -> World {
    let mut world = World::new();
    {
        let renderer = CircleRenderer {
            radius: 100.0,
            color: "#ff0000"
        };
        let visual = Visual::from_paintable(Box::new(renderer));
        let position = Position {
            x: 500.0,
            y: 300.0,
            angle: 0.0
        };
        let mass = Mass {
            mass: 500.0
        };
        world.spawn((visual, position, mass));
    }
    {
        let renderer = CircleRenderer{
            radius: 20.0,
            color: "#00ff00"
        };
        let visual = Visual::from_paintable(Box::new(renderer));
        let position = Position {
            x: 650.0,
            y: 300.0,
            angle: 0.0,
        };
        let velocity = Velocity {
            x: 0.0,
            y: 50.0
        };
        let spaceship = SpaceShip {
            angle: 0.0
        };
        world.spawn((visual, position, velocity, spaceship));
    }
    world
}

#[derive(Default)]
pub struct Input {
    forward: bool,
    brake: bool,
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
    let delta = now.duration_since(prev);
    {
        let input = input.borrow();

        system_spacecraft_input(&mut world, &input, delta);
        system_gravity(&mut world, delta);
        system_renderer(&mut world, &context, &store);
    }
    request_animation_frame(move ||{
        gloop(context, world, input, store, now);
    });
}
