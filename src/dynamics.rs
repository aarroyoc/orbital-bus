use hecs::{With, World};

use crate::{SpaceShip, Finish};

#[derive(Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

pub struct Velocity {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone)]
pub struct Celestial {
    pub mass: f64,
    pub radius: f64,
}

pub fn system_gravity(world: &mut World, delta: f64) {
    let planets: Vec<(Position, Celestial)> = world.query::<(&Position, &Celestial)>().iter().map(|(_id, (pos, cel))|{
        (pos.clone(), cel.clone())
    }).collect();
    
    for (_id, (position, velocity)) in &mut world.query::<(&mut Position, &mut Velocity)>() {
        for (celestial_position, celestial) in &planets {
            velocity.x += (celestial_position.x-position.x) * delta * celestial.mass;
            velocity.y += (celestial_position.y-position.y) * delta * celestial.mass;
        }
        // Roce
        //velocity.x *= 0.99;
        //velocity.y *= 0.99;
        position.x += velocity.x * delta;
        position.y += velocity.y * delta;
    }
}

pub fn system_crash(world: &mut World) {
    for (_id, spaceship_position) in &mut world.query::<With<SpaceShip, &Position>>() {
        for (_id, (position, celestial)) in &mut world.query::<(&Position, &Celestial)>() {
            if ((position.x - spaceship_position.x).powf(2.0) + (position.y - spaceship_position.y).powf(2.0)).sqrt() < celestial.radius {
                for (_id, finish) in &mut world.query::<&mut Finish>(){
                    finish.crash = true;
                }
            }
        }
    }
}