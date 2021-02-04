use crate::{ImageStore, SpaceShip, EndZone};
use crate::dynamics::{Celestial, Position, Velocity};
use crate::renderer::{Renderer};

pub fn background(mut store: &mut ImageStore) -> (Renderer, Position) {
    let mut renderer = Renderer::sprite("space.png", &mut store);
    renderer.set_z(-100);
    renderer.set_fixed(true);
    let position = Position {
        x: 0.0,
        y: 0.0,
    };
    (renderer, position)
}

pub fn spaceship(x: f64, y: f64, vx: f64, vy: f64, fuel: f64, mut store: &mut ImageStore) -> (Renderer, Position, Velocity, SpaceShip) {
    let renderer = Renderer::sprite("spaceship.png", &mut store);
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

pub fn earth(x: f64, y: f64, mut store: &mut ImageStore) -> (Renderer, Position, Celestial) {
    let renderer = Renderer::sprite("earth.png", &mut store);
    let position = Position {
        x,
        y,
    };
    let planet = Celestial {
        mass: 5.0,
        radius: 100.0,
    };
    (renderer, position, planet)
}

pub fn end_zone(x: f64, y: f64) -> (Renderer, Position, EndZone) {
    let renderer = Renderer::rect(100.0, 100.0, "rgba(250, 126, 55, 0.7)");
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