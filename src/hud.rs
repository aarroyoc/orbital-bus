use hecs::{With, World};

use crate::SpaceShip;
use crate::renderer::Renderer;
use crate::dynamics::Position;

pub struct FuelHUD;

pub fn system_hud(world: &mut World) {
    for (_id, spaceship) in &mut world.query::<&SpaceShip>() {
        for (_id, (renderer, position)) in &mut world.query::<With<FuelHUD, (&mut Renderer, &mut Position)>>(){
            if let Renderer::RectRenderer{height, ..} = renderer {
                *height = f64::max((spaceship.fuel*96.0)/spaceship.initial_fuel, 0.0);
                position.y = 653.0 +(96.0 - *height);
            }
            
        }
    }
}