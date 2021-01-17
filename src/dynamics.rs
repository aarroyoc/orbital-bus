use hecs::World;

pub struct Position {
    pub x: f64,
    pub y: f64,
}

pub struct Velocity {
    pub x: f64,
    pub y: f64,
}

pub struct Mass {
    pub mass: f64,
}

pub fn system_gravity(world: &mut World, delta: f64) {
    for (_id, (position, velocity)) in &mut world.query::<(&mut Position, &mut Velocity)>() {
        for (_id, (mass_position, mass)) in &mut world.query::<(&Position, &Mass)>() {
            velocity.x += (mass_position.x-position.x) * delta * mass.mass;
            velocity.y += (mass_position.y-position.y) * delta * mass.mass;
        }
        // Roce
        //velocity.x *= 0.99;
        //velocity.y *= 0.99;
        position.x += velocity.x * delta;
        position.y += velocity.y * delta;
    }
}