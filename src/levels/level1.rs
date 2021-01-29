use std::collections::HashMap;

use hecs::World;

use crate::dynamics::{Mass, Position, Velocity};
use crate::hud::FuelHUD;
use crate::renderer::{Camera, Renderer};
use crate::{EndZone, SpaceShip, Finish};

pub fn world_level_1(mut store: &mut HashMap<&'static str, web_sys::HtmlImageElement>) -> World {
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
    {
        let mut renderer = Renderer::sprite("hud.png", &mut store);
        renderer.set_fixed(true);
        renderer.set_z(8);
        let position = Position {
            x: 1230.0,
            y: 630.0,
        };
        world.spawn((renderer, position));

        let mut renderer = Renderer::sprite("can.png", &mut store);
        renderer.set_fixed(true);
        renderer.set_z(10);
        let position = Position {
            x: 1250.0,
            y: 650.0,
        };
        world.spawn((renderer, position));

        let mut renderer = Renderer::rect(77.0, 96.0, "red");
        renderer.set_fixed(true);
        renderer.set_z(9);
        let position = Position {
            x: 1252.0,
            y: 653.0,
        };
        world.spawn((renderer, position, FuelHUD));
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
            fuel: 25.0,
            initial_fuel: 25.0
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
    {
        let finish = Finish {
            finish: false
        };
        world.spawn((finish,));
    }
    world
}