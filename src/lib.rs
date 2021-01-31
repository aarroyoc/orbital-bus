use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use instant::Instant;
use hecs::*;

mod dynamics;
mod input;
mod hud;
mod levels;
mod renderer;
mod web;

use input::*;
use dynamics::*;
use hud::*;
use renderer::*;
use web::*;

struct SpaceShip {
    angle: f64,
    fuel: f64,
    initial_fuel: f64,
}

struct EndZone {
    width: f64,
    height: f64,
}

struct Finish {
    finish: bool,
}

const KEY_W: u32 = 87;
const KEY_S: u32 = 83;
const ORBITAL_BUS_LEVEL: &'static str = "orbital-bus-level";
const ORBITAL_BUS_MAX_LEVEL: &'static str = "orbital-bus-max-level";

fn system_finish(world: &mut World, mut input: &mut Input) {
    let mut show_win = false;
    for (_id, finish) in &mut world.query::<&mut Finish>(){
        if !finish.finish {
            for (_id, position) in &mut world.query::<With<SpaceShip, &Position>>() {
                for (_id, (zone_limits, zone_position)) in &mut world.query::<(&EndZone, &Position)>(){
                    if zone_position.x+20.0 < position.x && position.x < zone_position.x+zone_limits.width-20.0 {
                        if zone_position.y+20.0 < position.y && position.y < zone_position.y + zone_limits.height-20.0 {
                            finish.finish = true;
                            show_win = true;
                            input.click = false;
                        }
                    }
                }
            }
        } else if input.click {
            let level: i32 = get_local_storage(ORBITAL_BUS_LEVEL).parse().unwrap();
            let max_level: i32 = get_local_storage(ORBITAL_BUS_MAX_LEVEL).parse().unwrap();
            if max_level == level {
                let new_level = format!("{}", level + 1);
                set_local_storage(ORBITAL_BUS_MAX_LEVEL, &new_level);
            }
            go_web("index.html");
            input.click = false;
        }
    }
    
    if show_win {
        let mut rect = Renderer::rect(230.0, 75.0, "#1b1b1b");
        rect.set_fixed(true);
        rect.set_z(9);
        let mut text = Renderer::text(String::from("Success!"), "white", "40px Tsoonami");
        text.set_fixed(true);
        text.set_z(10);
        let mut click_text = Renderer::text(String::from("Click to continue"), "white", "14px Tsoonami");
        click_text.set_fixed(true);
        click_text.set_z(10);
        let position = Position {
            x: 590.0,
            y: 350.0,
        };
        let mut p = position.clone();
        p.x -= 5.0;
        p.y -= 40.0;
        let mut t = position.clone();
        t.x += 20.0;
        t.y += 25.0;
        world.spawn((text, position));
        world.spawn((rect, p));
        world.spawn((click_text, t));

    }
}

#[wasm_bindgen(start)]
pub fn start() {
    log("Welcome to Orbital Bus!");
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
    let level: i32 = get_local_storage(ORBITAL_BUS_LEVEL).parse().expect("orbital-bus-level is not a number");
    let world = levels::load_level(level, &mut store);

    let input = Rc::new(RefCell::new(Input::default()));
    let input_handler = input.clone();
    let click_handler = Closure::wrap(Box::new(move |event: web_sys::MouseEvent|{
        let mut input = input_handler.borrow_mut();
        let canvas = document().get_element_by_id("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let (x, y) = get_position(canvas, event);
        if y > 10.0 && y < 210.0 {
            if x > 10.0 && x < 110.0 {
                go_web("index.html");
            }
            if x > 110.0 && x < 210.0 {
                window().location().reload().unwrap();
            }
        } else {
            input.click = true;
        }
    }) as Box<dyn Fn(_)>);
    let input_handler = input.clone();
    let down_handler = Closure::wrap(Box::new(move |event|{
        let mut input = input_handler.borrow_mut();
        let canvas = document().get_element_by_id("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let (x, y) = get_position(canvas, event);
        if x > 10.0 && x < 260.0 {
            if y > 500.0 && y < 692.0 {
                input.forward = true;
            }
            if y > 692.0 && y < 784.0 {
                input.brake = true;
            }
        }
        let msg = format!("Pos: {}, {}", x, y);
        log(&msg);
    }) as Box<dyn Fn(_)>);
    let input_handler = input.clone();
    let up_handler = Closure::wrap(Box::new(move |event|{
        let mut input = input_handler.borrow_mut();
        let canvas = document().get_element_by_id("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let (x, y) = get_position(canvas, event);
        if x > 10.0 && x < 260.0 {
            if y > 500.0 && y < 692.0 {
                input.forward = false;
            }
            if y > 692.0 && y < 784.0 {
                input.brake = false;
            }
        }
        let msg = format!("Pos: {}, {}", x, y);
        log(&msg);
    }) as Box<dyn Fn(_)>);

    let input_handler = input.clone();
    let touchstart_handler = Closure::wrap(Box::new(move |event: web_sys::TouchEvent|{
        let mut input = input_handler.borrow_mut();
        let canvas = document().get_element_by_id("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let (x, y) = get_touch_position(canvas, event);
        if x > 10.0 && x < 260.0 {
            if y > 500.0 && y < 692.0 {
                input.forward = true;
            }
            if y > 692.0 && y < 784.0 {
                input.brake = true;
            }
        }
        let msg = format!("Pos: {}, {}", x, y);
        log(&msg);
        
    }) as Box<dyn Fn(_)>);

    let input_handler = input.clone();
    let touchend_handler = Closure::wrap(Box::new(move ||{
        let mut input = input_handler.borrow_mut();
        input.forward = false;
        input.brake = false;
    }) as Box<dyn Fn()>);

    let contextmenu_handler = Closure::wrap(Box::new(move |event: web_sys::Event|{
        event.prevent_default();
        event.stop_propagation();
    }) as Box<dyn Fn(_)>);

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

    window().set_onclick(Some(click_handler.as_ref().unchecked_ref()));
    window().set_onkeydown(Some(keydown_handler.as_ref().unchecked_ref()));
    window().set_onkeyup(Some(keyup_handler.as_ref().unchecked_ref()));
    canvas.set_onmousedown(Some(down_handler.as_ref().unchecked_ref()));
    canvas.set_onmouseup(Some(up_handler.as_ref().unchecked_ref()));
    canvas.set_ontouchstart(Some(touchstart_handler.as_ref().unchecked_ref()));
    canvas.set_ontouchend(Some(touchend_handler.as_ref().unchecked_ref()));
    canvas.set_oncontextmenu(Some(contextmenu_handler.as_ref().unchecked_ref()));

    click_handler.forget();
    keydown_handler.forget();
    keyup_handler.forget();
    down_handler.forget();
    up_handler.forget();
    touchstart_handler.forget();
    touchend_handler.forget();
    contextmenu_handler.forget();

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
            let mut input = input.borrow_mut();

            system_spacecraft_input(&mut world, &input, delta);
            system_gravity(&mut world, delta);
            system_finish(&mut world, &mut input);
            system_hud(&mut world);
            system_offset(&mut world);
            system_renderer(&mut world, &context, &store);
        }
    }
    request_animation_frame(move ||{
        gloop(context, world, input, store, now);
    });
}
