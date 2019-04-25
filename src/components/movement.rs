use specs::prelude::*;

use std::time::Duration;

use nalgebra::{ Vector2 };

type Num = f32;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    loc: Vector2<Num>
}

impl Position {
    pub fn new(x: Num, y: Num) -> Self {
        Position { loc: Vector2::new(x, y) }
    }

    pub fn accumulate_v(&mut self, v: &Velocity, t: f32) {
        let percent_v = t / v.delta_t;

        let adjusted_displacement = v.displacement * percent_v;

        self.loc += adjusted_displacement;
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
// Represented by a Vector representing displacement in meters
// and a delta_t in seconds
pub struct Velocity {
    displacement: Vector2<Num>,
    delta_t: f32
}

impl Velocity {
    pub fn meters_per_second(x: Num, y: Num) -> Self {
        Velocity {
            displacement: Vector2::new(x, y),
            delta_t: 1.0
        }
    }
}