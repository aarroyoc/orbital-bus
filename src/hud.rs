use hecs::{With, World};

use crate::{SpaceShip, Finish, ImageStore};
use crate::renderer::{Renderer, Camera};
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

pub fn build_hud(world: &mut World, mut store: &mut ImageStore) {
    {
        let mut renderer = Renderer::sprite("hud.png".to_string(), &mut store);
        renderer.set_fixed(true);
        renderer.set_z(8);
        let position = Position {
            x: 1230.0,
            y: 630.0,
        };
        world.spawn((renderer, position));

        let mut renderer = Renderer::sprite("can.png".to_string(), &mut store);
        renderer.set_fixed(true);
        renderer.set_z(10);
        let position = Position {
            x: 1250.0,
            y: 650.0,
        };
        world.spawn((renderer, position));

        let mut renderer = Renderer::rect(77.0, 96.0, "red".to_string());
        renderer.set_fixed(true);
        renderer.set_z(9);
        let position = Position {
            x: 1252.0,
            y: 653.0,
        };
        world.spawn((renderer, position, FuelHUD));
    }
    {
        let mut renderer = Renderer::sprite("controls.png".to_string(), &mut store);
        renderer.set_fixed(true);
        renderer.set_z(10);
        let position = Position {
            x: 10.0,
            y: 500.0,
        };
        world.spawn((renderer, position));
    }
    {
        let mut renderer = Renderer::sprite("back-restart.png".to_string(), &mut store);
        renderer.set_fixed(true);
        renderer.set_z(10);
        let position = Position {
            x: 10.0,
            y: 10.0,
        };
        world.spawn((renderer, position));
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
        let finish = Finish::default();
        world.spawn((finish,));
    }
}