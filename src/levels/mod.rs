use serde::{Deserialize};

use crate::ImageStore;

mod common;

#[derive(Deserialize)]
struct PlanetDef {
    id: String,
    sprite: String,
    radius: f64,
    mass: f64,
    fix: bool,
}

#[derive(Deserialize)]
struct Planet {
    r#ref: String, 
    x: f64,
    y: f64,
}

#[derive(Deserialize)]
struct Text {
    text: String,
    color: String,
    style: String,
    x: f64,
    y: f64,
}

#[derive(Deserialize)]
struct SpaceShip {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    fuel: f64
}

#[derive(Deserialize)]
struct End {
    x: f64,
    y: f64
}

#[derive(Deserialize)]
struct World {
    id: i32,
    background: bool,
    planets: Vec<Planet>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    texts: Option<Vec<Text>>,
    spaceship: SpaceShip,
    end: End,
}

#[derive(Deserialize)]
struct Root {
    worlds: Vec<World>,
    planets: Vec<PlanetDef>,
}

const LEVELS_STRING: &'static str = include_str!("levels.ron");

pub fn load_level(level: i32, mut store: &mut ImageStore) -> hecs::World {
    let mut world = hecs::World::new();
    let levels = load_levels();
    if let Some(level) = levels.worlds.iter().find(|x| x.id == level) {
        if level.background {
            world.spawn(common::background(&mut store));
        }

        for planet in &level.planets {
            if let Some(planet_def) = levels.planets.iter().find(|x| x.id == planet.r#ref) {
                if planet_def.fix {
                    world.spawn(common::fixplanet(&planet_def.sprite, planet.x, planet.y, planet_def.mass, planet_def.radius, &mut store));
                } else {
                    world.spawn(common::planet(&planet_def.sprite, planet.x, planet.y, 0.0, 400.0, planet_def.mass, planet_def.radius, &mut store));
                }
            }
        }

        if let Some(texts) = &level.texts {
            for text in texts {
                world.spawn(common::text(text.text.clone(), text.color.clone(), text.style.clone(), text.x, text.y));
            }
        }

        world.spawn(common::spaceship(level.spaceship.x, level.spaceship.y, level.spaceship.vx, level.spaceship.vy, level.spaceship.fuel, &mut store));
        world.spawn(common::end_zone(level.end.x, level.end.y));
    }

    crate::hud::build_hud(&mut world, &mut store);
    world
}

fn load_levels() -> Root {
    ron::from_str(LEVELS_STRING).expect("levels file is corrupted")
}