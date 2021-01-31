use std::collections::HashMap;

use hecs::World;

use crate::dynamics::{Mass, Position, Velocity};
use crate::renderer::{Renderer};
use crate::{EndZone, SpaceShip};

pub fn world_level(mut store: &mut HashMap<&'static str, web_sys::HtmlImageElement>) -> World {
    let mut world = World::new();
    {
        let mut renderer = Renderer::sprite("space.png", &mut store);
        renderer.set_z(-100);
        renderer.set_fixed(true);
        let position = Position {
            x: 0.0,
            y: 0.0,
        };
        world.spawn((renderer, position));
    }
    {
        let renderer = Renderer::sprite("earth.png", &mut store);
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
        let renderer = Renderer::sprite("spaceship.png", &mut store);
        let position = Position {
            x: 800.0,
            y: 400.0,
        };
        let velocity = Velocity {
            x: 0.0,
            y: 300.0
        };
        let spaceship = SpaceShip {
            angle: 0.0,
            fuel: 10.0,
            initial_fuel: 10.0
        };
        world.spawn((renderer, position, velocity, spaceship));
    }
    crate::hud::build_hud(&mut world, &mut store);
    world
}