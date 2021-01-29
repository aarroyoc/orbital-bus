use std::collections::HashMap;

use hecs::World;

mod level1;

pub fn load_level(level: i32, mut store: &mut HashMap<&'static str, web_sys::HtmlImageElement>) -> World {
    match level {
        1 => level1::world_level_1(&mut store),
        _ => panic!("No level found!")
    }
}