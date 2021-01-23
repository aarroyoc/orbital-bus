use hecs::World;

use crate::SpaceShip;
use crate::dynamics::Velocity;

const MAX_SPEED: f64 = 1000.0;
const SPEED_INCREMENT: f64 = 1000.0;
const SPEED_DECREMENT: f64 = 600.0;

#[derive(Default)]
pub struct Input {
    pub forward: bool,
    pub brake: bool,
}

pub fn system_spacecraft_input(world: &mut World, input: &Input, delta: f64) {
    for (_id, (spaceship, velocity)) in &mut world.query::<(&mut SpaceShip, &mut Velocity)>() {
        spaceship.angle = velocity.y.atan2(velocity.x);
        let mut x = velocity.x;
        let mut y = velocity.y;
        if spaceship.fuel > 0.0 {
            if input.forward {
                x += SPEED_INCREMENT * delta * spaceship.angle.cos();
                y += SPEED_INCREMENT * delta * spaceship.angle.sin();
                spaceship.fuel -= delta*10.0;
            }
            if input.brake {
                x -= SPEED_DECREMENT * delta * spaceship.angle.cos();
                y -= SPEED_DECREMENT * delta * spaceship.angle.sin();
                spaceship.fuel -= delta*10.0;
            }
            
        }
        let speed = (x * x + y * y).sqrt();
        if speed < MAX_SPEED {
            velocity.x = x;
            velocity.y = y;
        }
    }
    
}