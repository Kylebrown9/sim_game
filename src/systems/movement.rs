use specs::{ Read, ReadStorage, System, WriteStorage, Join };

use crate::components::{ Velocity, Position };
use crate::resources::{ DeltaTime };

pub struct PosPrinter;

impl<'a> System<'a> for PosPrinter {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        for position in position.join() {
            println!("{:?}", &position);
        }
    }
}

pub struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (Read<'a, DeltaTime>,
                       ReadStorage<'a, Velocity>,
                       WriteStorage<'a, Position>);

    fn run(&mut self, (delta, velocities, mut positions): Self::SystemData) {
        // `Read` implements `Deref`, so it
        // coerces to `&DeltaTime`.
        let delta = delta.0;

        for (vel, pos) in (&velocities, &mut positions).join() {
            pos.accumulate_v(vel, delta);
        }
    }
}