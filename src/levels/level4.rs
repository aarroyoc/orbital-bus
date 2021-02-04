use hecs::World;

use crate::ImageStore;
use crate::levels::common::*;

pub fn world_level(mut store: &mut ImageStore) -> World {
    let mut world = World::new();
    world.spawn(background(&mut store));
    world.spawn(earth(650.0, 400.0, &mut store));
    world.spawn(spaceship(650.0, 700.0, 400.0, 0.0, 15.0, &mut store));
    world.spawn(end_zone(1200.0, 400.0));
    world
}