use hecs::World;

use crate::ImageStore;
use crate::levels::common::*;

pub fn world_level(mut store: &mut ImageStore) -> World {
    let mut world = World::new();
    world.spawn(background(&mut store));
    world.spawn(earth(650.0, 400.0, &mut store));
    world.spawn(spaceship(1100.0, 400.0, 0.0, 300.0, 12.0, &mut store));
    world.spawn(end_zone(800.0, 400.0));
    world
}