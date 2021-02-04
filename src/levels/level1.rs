use hecs::World;

use crate::ImageStore;
use crate::dynamics::{Celestial, Position, Velocity};
use crate::renderer::{Renderer};
use crate::{EndZone, SpaceShip};
use crate::levels::common::*;

pub fn world_level(mut store: &mut ImageStore) -> World {
    let mut world = World::new();
    world.spawn(background(&mut store));
    {
        let renderer = Renderer::sprite("earth.png", &mut store);
        let position = Position {
            x: 650.0,
            y: 400.0,
        };
        let planet = Celestial {
            mass: 5.0,
            radius: 100.0,
        };
        world.spawn((renderer, position, planet));
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
        let mut renderer = Renderer::text(String::from("Welcome to Orbital Bus"), "white", "20pt Tsoonami");
        renderer.set_fixed(true);
        renderer.set_z(10);
        let position = Position {
            x: 480.0,
            y: 100.0,
        };
        world.spawn((renderer, position));
    }
    {
        let mut renderer = Renderer::text(String::from("The objective is simple. Try to go to the square zone"), "white", "15pt Tsoonami");
        renderer.set_fixed(true);
        renderer.set_z(10);
        let position = Position {
            x: 370.0,
            y: 130.0
        };
        world.spawn((renderer, position));
    }
    {
        let mut renderer = Renderer::text(String::from("Use the keys (W,S) or the on-screen buttons to accelerate and brake"), "white", "15pt Tsoonami");
        renderer.set_fixed(true);
        renderer.set_z(10);
        let position = Position {
            x: 280.0,
            y: 150.0,
        };
        world.spawn((renderer, position));
    }
    {
        let mut renderer = Renderer::text(String::from("Be careful! You have limited fuel"), "white", "15pt Tsoonami");
        renderer.set_fixed(true);
        renderer.set_z(10);
        let position = Position {
            x: 470.0,
            y: 170.0
        };
        world.spawn((renderer, position));
    }
    world.spawn(spaceship(800.0, 400.0, 0.0, 300.0, 25.0, &mut store));
    world
}