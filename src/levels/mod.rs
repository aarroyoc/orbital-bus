use std::collections::HashMap;

use hecs::World;

mod level1;
mod level2;

pub fn load_level(level: i32, mut store: &mut HashMap<&'static str, web_sys::HtmlImageElement>) -> World {
    let mut world = match level {
        1 => level1::world_level(&mut store),
        2 => level2::world_level(&mut store),
        _ => panic!("No level found!")
    };
    crate::hud::build_hud(&mut world, &mut store);
    world
}