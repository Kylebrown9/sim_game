extern crate nalgebra;
extern crate specs;
#[macro_use]
extern crate specs_derive;

mod components;
mod resources;
mod systems;

use crate::components::{ Position, Velocity };
use crate::resources::{ DeltaTime };
use crate::systems::{ PosPrinter, UpdatePos };

use specs::prelude::*;
use specs::{ World, Builder };

fn main() {
    let mut world = World::new();

    world.register::<Position>();
    world.register::<Velocity>();

    world.add_resource(DeltaTime::new(1, 0));

    let ball = world.create_entity().with(Position::new(0.0, 0.0)).build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(UpdatePos, "update_position", &[])
        .with(PosPrinter, "position_printer", &["update_position"])
        .build();
}
