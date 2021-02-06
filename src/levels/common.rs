use crate::{ImageStore, SpaceShip, EndZone};
use crate::dynamics::{Celestial, Position, Velocity};
use crate::renderer::{Renderer};

pub fn background(mut store: &mut ImageStore) -> (Renderer, Position) {
    let mut renderer = Renderer::sprite("space.png".to_string(), &mut store);
    renderer.set_z(-100);
    renderer.set_fixed(true);
    let position = Position {
        x: 0.0,
        y: 0.0,
    };
    (renderer, position)
}

pub fn spaceship(x: f64, y: f64, vx: f64, vy: f64, fuel: f64, mut store: &mut ImageStore) -> (Renderer, Position, Velocity, SpaceShip) {
    let renderer = Renderer::sprite("spaceship.png".to_string(), &mut store);
    let position = Position {
        x,
        y,
    };
    let velocity = Velocity {
        x: vx,
        y: vy
    };
    let spaceship = SpaceShip {
        angle: 0.0,
        fuel,
        initial_fuel: fuel
    };
    (renderer, position, velocity, spaceship)
}

pub fn planet(sprite: &str, x: f64, y: f64, mass: f64, radius: f64, mut store: &mut ImageStore) -> (Renderer, Position, Celestial) {
    let renderer = Renderer::sprite(sprite.to_string(), &mut store);
    let position = Position {
        x,
        y,
    };
    let planet = Celestial {
        mass,
        radius,
    };
    (renderer, position, planet)
}

pub fn end_zone(x: f64, y: f64) -> (Renderer, Position, EndZone) {
    let renderer = Renderer::rect(100.0, 100.0, "rgba(250, 126, 55, 0.7)".to_string());
    let position = Position {
        x: x-50.0,
        y: y-50.0,
    };
    let end = EndZone {
        width: 100.0,
        height: 100.0,
    };
    (renderer, position, end)
}

pub fn text(text: String, color: String, style: String, x: f64, y: f64) -> (Renderer, Position) {
    let mut renderer = Renderer::text(text, color, style);
    renderer.set_fixed(true);
    renderer.set_z(10);
    let position = Position {
        x,
        y
    };
    (renderer, position)
}