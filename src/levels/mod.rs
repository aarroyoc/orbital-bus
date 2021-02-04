use std::collections::HashMap;

use hecs::World;

mod common;
mod level1;
mod level2;
mod level3;
mod level4;
mod level5;

pub fn load_level(level: i32, mut store: &mut HashMap<&'static str, web_sys::HtmlImageElement>) -> World {
    let mut world = match level {
        1 => level1::world_level(&mut store),
        2 => level2::world_level(&mut store),
        3 => level3::world_level(&mut store),
        4 => level4::world_level(&mut store),
        5 => level5::world_level(&mut store),
        _ => panic!("No level found!")
    };
    crate::hud::build_hud(&mut world, &mut store);
    world
}